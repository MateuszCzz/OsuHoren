export interface SongData {
  path: string;
  audio: string;

  title: string;
  unicodeTitle: string;
  artist: string;
  unicodeArtist: string;
  mapper: string;

  tags: string;
  setId: string;
  source: string;

  audioFile: File;
}
