import type { SongType } from "../types/song-type";

self.onmessage = async (e) => {
  // how many songs per package to send back
  const fileList: File[] = e.data;
  const batchSize = 500;

  // get files ending with .osu
  const osuFiles = fileList.filter((f) => f.name.endsWith(".osu"));

  // no osu files found
  if (!osuFiles.length) {
    postMessage({ error: true, fatal: true, message: "No songs detected in folder." });
    return;
  }

  // remove diff brackets to remove duplicates 
  const unique = new Map<string, File>();
  for (const f of osuFiles) {
    const base = f.name.replace(/ \[.*\](?=\.osu$)/, "").toLowerCase();
    if (!unique.has(base)) unique.set(base, f);
  }

  postMessage({
    processing: true,
    unique_amount: unique.size,
  });

  //TODO: FILTER OUT SAME TITLE AUTOR LENGTH SONGS

  // main processing:
  let batch: SongType[] = [];

  for (const osu of unique.values()) {
    const metadata = await parseOsuFile(osu);

    // no audio surce in .osu file, skip
    if (!metadata) {
      postMessage({
        error: true,
        fatal: false,
        message: `No audio source in .osu file: ${osu.name}`,
      });
      continue;
    }

    // look for coresponding with .osu metadata, audio files
    const basePath = osu.webkitRelativePath.split("/").slice(0, -1).join("/");
    const audioFile = fileList.find((f) => f.webkitRelativePath === `${basePath}/${metadata.audio}`);

    // audio file wasnt in directory, skip
    if (!audioFile) {
      postMessage({
        error: true,
        fatal: false,
        message: `Audio file not found at ${basePath}/${metadata.audio}`,
      });
      continue;
    }

    // add to stack 
    // id handled by song store
    batch.push({
      id: 0,
      path: osu.webkitRelativePath,
      ...metadata,
      audioFile,
    } as SongType);

    // when stack is big enough send over
    if (batch.length >= batchSize) {
      postMessage({ partial: true, data: batch });
      batch = [];
    }
  }

  // send left overs
  if (batch.length) {
    postMessage({ partial: true, data: batch });
  }

  postMessage({ done: true });
};

// helper, parse metadata from .osu file
async function parseOsuFile(file: File) {
  const text = await file.text();
  const metadata: any = {};

  for (const line of text.split(/\r?\n/)) {
    if (line.startsWith("[TimingPoints]")) break;

    if (line.startsWith("AudioFilename:")) {
      metadata.audio = line.slice(14).trim();
    } else if (line.startsWith("Title:")) {
      metadata.title = line.slice(6).trim();
    } else if (line.startsWith("Artist:")) {
      metadata.artist = line.slice(7).trim();
    } else if (line.startsWith("TitleUnicode:")) {
      metadata.unicodeTitle = line.slice(13).trim();
    } else if (line.startsWith("ArtistUnicode:")) {
      metadata.unicodeArtist = line.slice(14).trim();
    } else if (line.startsWith("Creator:")) {
      metadata.mapper = line.slice(8).trim();
    } else if (line.startsWith("Source:")) {
      metadata.source = line.slice(7).trim();
    } else if (line.startsWith("Tags:")) {
      metadata.tags = line.slice(5).trim();
    } else if (line.startsWith("BeatmapSetID:")) {
      metadata.setId = line.slice(13).trim();
    }
  }

  // return empty on no audio tag
  return metadata.audio ? metadata : null;
}
