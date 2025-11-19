use chrono::{DateTime, Utc};
use futures::{
    channel::mpsc::{self, Sender},
    StreamExt,
};
use osynic_osudb::entity::osu::osudb::OsuDB;
use serde::Serialize;
use std::{fs, path::PathBuf};
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
}

// read on Arc for cheap cloning
async fn songs_processor_worker(
    songs_dir_path: PathBuf,
    osu_db: OsuDB,
    mut sender: Sender<SongData>,
) {
    sender.disconnect();
}
