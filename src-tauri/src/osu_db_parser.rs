use osynic_osudb::entity::osu::osudb::OsuDB;
use serde::Serialize;
use std::fs;
use tauri::{AppHandle, Emitter};
use tauri_plugin_dialog::DialogExt;
// use tauri::ipc::Channel;

#[derive(Clone, Serialize)]
pub struct FilePickerResult {
    path: Option<String>,
    error: Option<String>,
    warning: Option<String>,
    songs: Option<String>,
}

const ERR_NO_FILE_SELECTED: &str = "No file selected";
const WAR_WRONG_FILE_NAME: &str = "Expected selected file to be called osu!.db";
const ERR_PARSE_FAILED: &str = "Failed to parse osu!.db";
const WAR_SONG_FOLDER_WRONG: &str =
    "Songs folder not found or empty. Please select proper Songs folder.";
const ERR_NO_SONG_INFO: &str = "No Song informations found inside the osu!db file";
const WAR_MISS_COUNT: &str = "Songs folder holds diffrent amount of songs than osu!db file.";

/// helper to quickly create a hard error `FilePickerResult`
fn handle_fail(path: Option<&std::path::Path>, msg: impl Into<String>) -> FilePickerResult {
    FilePickerResult {
        path: path.map(|p| p.to_string_lossy().into_owned()),
        error: Some(msg.into()),
        warning: None,
        songs: None,
    }
}

#[tauri::command]
pub fn load_osu_db(app: AppHandle) {
    // open file picker dialog
    let pick_file = app
        .dialog()
        .file()
        .set_title("Select osu!.db from the game directory")
        .set_file_name("osu!.db")
        .add_filter("File format filter", &["db"])
        .blocking_pick_file();

    // was file selected
    let file_path = match pick_file {
        Some(path) => path,
        // err: no file
        None => {
            app.emit(
                "file-picker-result",
                handle_fail(None, ERR_NO_FILE_SELECTED),
            )
            .unwrap();
            return;
        }
    };

    // transform path
    let osu_db_path = file_path.as_path().unwrap();

    // is it called osu!.db
    if !osu_db_path.ends_with("osu!.db") {
        //war: wrong file name
        app.emit(
            "file-picker-result",
            FilePickerResult {
                path: Some(osu_db_path.to_string_lossy().into_owned()),
                error: None,
                warning: Some(WAR_WRONG_FILE_NAME.into()),
                songs: None,
            },
        )
        .unwrap();
    }

    // try to parse with osu_db
    let osu_db = match OsuDB::from_file(osu_db_path) {
        Ok(db) => db,
        // err: file doesnt follow osu standard db structure
        Err(_) => {
            app.emit(
                "file-picker-result",
                handle_fail(Some(osu_db_path), ERR_PARSE_FAILED),
            )
            .unwrap();
            return;
        }
    };

    // get amount of unique songs from db
    let osu_db_count = osu_db.folder_count;
    // check if any songs in db
    if osu_db_count == 0 {
        // err: empty osu!.db
        app.emit(
            "file-picker-result",
            handle_fail(Some(osu_db_path), ERR_NO_SONG_INFO),
        )
        .unwrap();
        return;
    };

    // assume song dir path based on db
    let mut song_dir_path = osu_db_path.parent().unwrap().join("Songs");
    let mut song_dir_path_count: usize;

    // loop until user gives song folder with at least 1 dir inside
    loop {
        // try to read content of songs dir
        song_dir_path_count = match fs::read_dir(&song_dir_path) {
            Ok(entries) => entries
                .filter_map(|e| e.ok())
                .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
                .count(),
            Err(_) => 0,
        };

        if song_dir_path_count > 0 {
            //  successful, break out of the loop
            break;
        } else {
            // war: songs folder failed
            app.emit(
                "file-picker-result",
                FilePickerResult {
                    path: Some(osu_db_path.to_string_lossy().into_owned()),
                    error: None,
                    warning: Some(WAR_SONG_FOLDER_WRONG.into()),
                    songs: None,
                },
            )
            .unwrap();

            // repick songs folder
            let pick_file = app
                .dialog()
                .file()
                .set_title("Select Songs folder from the game directory")
                .blocking_pick_folder();

            let file_path = match pick_file {
                Some(path) => path,
                None => {
                    app.emit(
                        "file-picker-result",
                        handle_fail(None, ERR_NO_FILE_SELECTED),
                    )
                    .unwrap();
                    return;
                }
            };

            song_dir_path = file_path.into_path().unwrap();
        }
    }

    if osu_db_count as usize != song_dir_path_count {
        // war: miss count between songs in db and songs folders
        app.emit(
            "file-picker-result",
            FilePickerResult {
                path: Some(osu_db_path.to_string_lossy().into_owned()),
                error: None,
                warning: Some(WAR_MISS_COUNT.into()),
                songs: None,
            },
        )
        .unwrap();
    };

    //    all is good
}

