use chrono::{DateTime, Utc};
use futures::{
    channel::mpsc::{self, Sender},
    stream, SinkExt, StreamExt,
};
use osynic_osudb::entity::osu::osudb::OsuDB;
use serde::Serialize;
use std::{collections::HashSet, fs, path::PathBuf};
use tauri::{async_runtime, AppHandle, Emitter};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

#[derive(Clone, Serialize)]
pub struct DataSetupResult {
    status: Status,
    message: Option<String>,
    songs: Option<Vec<SongData>>,
}

#[derive(Clone, Serialize)]
pub enum Status {
    Warning,    // soft error
    Error,      // hard error program finished
    Started,    // started processing
    Processing, // partial data
    Done,       // finished stop listing for more output
}

#[derive(Clone, Serialize)]
pub struct SongData {
    artist_ascii: Option<String>,
    artist_unicode: Option<String>,
    title_ascii: Option<String>,
    title_unicode: Option<String>,
    creator: Option<String>,
    song_source: Option<String>,
    tags: Option<String>,
    length: Option<u32>, // from total_time
    mode: Option<u8>,
    ranking_status: Option<u8>,
    last_modified: Option<DateTime<Utc>>,
}

const ERR_NO_FILE_SELECTED: &str = "No file selected";
const ERR_PATH_EXTRACT_FAILED: &str = "Failed to extract path from the selected file";
const ERR_OSU_DB_PARSE_FAILED: &str = "Failed to parse osu!.db";
const ERR_OSU_DB_EMPTY: &str = "No song information found inside osu!.db";
const ERR_SONGS_DIR_READ_FAILED: &str = "Songs folder is unreadable";
const ERR_SONGS_DIR_EMPTY: &str = "Songs folder is empty";
const EVENT_DATA_SETUP_RESULT: &str = "data-setup-result";
const WARN_MISMATCH_COUNT: &str =
    "Songs folder contains a different number of songs than the osu!.db file";
const SONGS_PROCESS_WORKER_BATCH_SIZE: usize = 500;

/// helper to quickly create DataSetupResult
fn make_dsr(status: Status, msg: impl Into<String>) -> DataSetupResult {
    DataSetupResult {
        status: status,
        message: Some(msg.into()),
        songs: None,
    }
}

/// helper to emit error
fn emit_fail(app: &AppHandle, msg: impl Into<String>) {
    let _ = app.emit(EVENT_DATA_SETUP_RESULT, make_dsr(Status::Error, msg));
}

/// helper to pick new songs folder
/// returns pathbuf on success
/// ERR_NO_FILE_SELECTED on no file selected
///ERR_PATH_EXTRACT_FAILED on failed to extract path from given file
fn pick_songs_folder(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    let folder = app
        .dialog()
        .file()
        .set_title("Select Songs folder from the game directory")
        .set_file_name("Songs")
        .blocking_pick_folder()
        .ok_or_else(|| ERR_NO_FILE_SELECTED.to_string())?;

    let path = folder
        .into_path()
        .map_err(|_| ERR_PATH_EXTRACT_FAILED.to_string())?;

    Ok(path)
}

#[tauri::command]
pub async fn setup_osu_data(app: AppHandle) {
    // open file picker dialog for osu!.db
    let Some(db_file_picker) = app
        .dialog()
        .file()
        .set_title("Select osu!.db from the game directory")
        .set_file_name("osu!.db")
        .add_filter("File format filter", &["db"])
        .blocking_pick_file()
    else {
        // error: No file selected
        emit_fail(&app, ERR_NO_FILE_SELECTED);
        return;
    };

    // extract path from selected file
    let osu_db_path = match db_file_picker.into_path() {
        Ok(p) => p,
        Err(_) => return emit_fail(&app, ERR_PATH_EXTRACT_FAILED),
    };

    // parse path into OsuDB using 'osynic_osudb'
    let osu_db = match OsuDB::from_file(&osu_db_path) {
        Ok(db) => db,
        Err(_) => return emit_fail(&app, ERR_OSU_DB_PARSE_FAILED),
    };

    // get amount of content in osu db
    let osu_db_count = osu_db.folder_count;
    if osu_db_count <= 0 {
        // error: osu db is empty
        emit_fail(&app, ERR_OSU_DB_EMPTY);
        return;
    }

    // assume default song path as sibling of osu_db
    let songs_dir_path = match osu_db_path.parent() {
        Some(parent_path) => {
            // append Songs for  ../Songs
            let def_path = parent_path.join("Songs");
            if def_path.exists() {
                def_path
            } else {
                // error: no such path
                // allow user to pick songs dir
                match pick_songs_folder(&app) {
                    Ok(p) => p,
                    Err(err_msg) => {
                        // error: non selected or failed to extract
                        emit_fail(&app, &err_msg);
                        return;
                    }
                }
            }
        }
        None => {
            // osu_db has no parent
            // allow user to pick songs dir
            match pick_songs_folder(&app) {
                Ok(p) => p,
                Err(err) => {
                    // error: non selected or failed to extract
                    emit_fail(&app, &err);
                    return;
                }
            }
        }
    };

    // count content of Songs dir
    let songs_dir_count = match fs::read_dir(&songs_dir_path) {
        Ok(rd) => rd
            // map children
            .filter_map(Result::ok)
            // filter out non dirs
            .filter(|entry| entry.file_type().map(|t| t.is_dir()).unwrap_or(false))
            .count(),
        Err(_) => {
            // error: failed to read
            emit_fail(&app, ERR_SONGS_DIR_READ_FAILED);
            return;
        }
    };

    if songs_dir_count <= 0 {
        // error: songs dir is empty
        emit_fail(&app, ERR_SONGS_DIR_EMPTY);
        return;
    }

    // if there are more songs in db than on drive
    // prompt user if process should be continued
    if songs_dir_count < osu_db_count as usize {
        // send warning
        app.emit(
            EVENT_DATA_SETUP_RESULT,
            make_dsr(Status::Warning, WARN_MISMATCH_COUNT),
        )
        .unwrap();

        // prompt user
        let continue_prompt = app
            .dialog()
            .message(format!("The number of songs in your osu!db ({}) does not match the number of songs in your Songs folder ({}). Continue anyway?
",osu_db_count, songs_dir_count))
            .kind(MessageDialogKind::Warning)
            .title("Songs Missmatch")
            .blocking_show();
        if !continue_prompt {
            // finish process on user prompt
            return;
        }
    }

    // inform frontend of started processing, give max amount of songs
    app.emit(
        EVENT_DATA_SETUP_RESULT,
        make_dsr(Status::Started, osu_db_count.to_string()),
    )
    .unwrap();

    // prepare connection channel, size of double the amount of buffor
    let (sender, mut receiver) = mpsc::channel::<SongData>(SONGS_PROCESS_WORKER_BATCH_SIZE * 2);

    // prep array for results
    let mut songs_processed: Vec<SongData> = vec![];
    let mut songs_result_count: usize = 0;

    // spawn async songs processing worker on new thread
    async_runtime::spawn(songs_processor_worker(songs_dir_path, osu_db, sender));

    // handle upcomming batches
    while let Some(song) = receiver.next().await {
        // get song into array
        songs_processed.push(song);
        songs_result_count += 1;

        // every buffor size, send amount of processed songs to frontend
        if songs_result_count >= SONGS_PROCESS_WORKER_BATCH_SIZE {
            songs_result_count = 0;
            app.emit(
                EVENT_DATA_SETUP_RESULT,
                make_dsr(Status::Processing, songs_processed.len().to_string()),
            )
            .unwrap();
        }
    }

    app.emit(
        EVENT_DATA_SETUP_RESULT,
        DataSetupResult {
            status: Status::Done,
            message: None,
            songs: Some(songs_processed),
        },
    )
    .unwrap();

    // save to global variable
    // songs_processed;
}

// read on Arc for cheap cloning
async fn songs_processor_worker(
    songs_dir_path: PathBuf,
    osu_db: OsuDB,
    mut sender: Sender<SongData>,
) {
    // set for holding processed songs names
    let mut done_songs: HashSet<String> = HashSet::new();
    let mut songs_batch: Vec<SongData> = Vec::with_capacity(SONGS_PROCESS_WORKER_BATCH_SIZE);

    // iter over songs in osu!db
    for beatmap in osu_db.beatmaps.iter() {
        // check if given song already processed
        let song_id = format!(
            "{} == {}",
            beatmap.artist_ascii.as_deref().unwrap_or("artist unknown"),
            beatmap.title_ascii.as_deref().unwrap_or("title unknown")
        );

        if done_songs.contains(&song_id) {
            // skip done already
            continue;
        }

        // prep folder or skip
        let folder = match beatmap.folder_name.as_ref() {
            Some(v) => v,
            None => {
                continue;
            }
        };

        // prep audio or skip
        let audio = match beatmap.audio.as_ref() {
            Some(v) => v,
            None => {
                continue;
            }
        };

        // prep full audio file path
        let audio_file_path = songs_dir_path.join(folder).join(audio);

        // look for audio file
        if fs::metadata(&audio_file_path).is_err() {
            continue;
        }

        done_songs.insert(song_id.clone());

        // prep whole song
        let song = SongData {
            artist_ascii: beatmap.artist_ascii.clone(),
            artist_unicode: beatmap.artist_unicode.clone(),
            title_ascii: beatmap.title_ascii.clone(),
            title_unicode: beatmap.title_unicode.clone(),
            creator: beatmap.creator.clone(),
            ranking_status: Some(beatmap.status.raw()),
            length: Some(beatmap.total_time.clone()),
            mode: Some(beatmap.mode.raw()),
            song_source: beatmap.song_source.clone(),
            tags: beatmap.tags.clone(),
            last_modified: Some(beatmap.last_modified),
        };

        // add song to batch
        songs_batch.push(song);

        // send batch if reached size
        if songs_batch.len() >= SONGS_PROCESS_WORKER_BATCH_SIZE {
            // send results from batch while draining
            let mut songs_stream = stream::iter(songs_batch.drain(..).map(Ok));
            sender.send_all(&mut songs_stream).await.unwrap();
        }
    }
    // send leftover songs
    let mut songs_stream = stream::iter(songs_batch.drain(..).map(Ok));
    sender.send_all(&mut songs_stream).await.unwrap();

    // close channel after complete
    sender.disconnect();
}
