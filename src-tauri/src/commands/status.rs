// [Sync]: must be synced with `src/pages/Status/index.svelte`

use std::fs;

use crate::util;

/// [Sync]
static LATEST_VERSION: &'static str = "LATEST_VERSION";
static GAME_PATH: &'static str = "GAME_PATH";

/// [Sync]
#[derive(serde::Serialize, Default, Clone)]
pub struct StatusData {
    latest_release_version: String,
    game_path: String,
}

#[tauri::command]
pub async fn status(req_type: String) -> StatusData {
    let mut status_data = StatusData::default();

    if req_type == LATEST_VERSION {
        status_data.latest_release_version = get_latest_release().await;
    } else if req_type == GAME_PATH {
        status_data.game_path = get_game_path()
    }

    return status_data;
}

/// get the latest WBM release version.
/// data is not converted to a json object because it'll be done in the front-end
async fn get_latest_release() -> String {
    let client = util::build_client();

    // todo: handle error
    let res = client
        .get("https://api.github.com/repos/War-Brokers-Mods/WBM/releases")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    return res;
}

/// gets path to WB game files. Return empty string if path was not found.
fn get_game_path() -> String {
    let game_path = match std::env::consts::OS {
        "linux" => "~/.steam/steam/SteamApps/common",
        "macos" => "~/Library/Application Support/Steam/steamapps/common",
        "windows" => "C:\\Program Files\\Steam\\steamapps\\common",

        _ => panic!("Unsupported platform!"),
    };

    // https://stackoverflow.com/a/32384768/12979111
    return String::from(if fs::metadata(game_path).is_ok() {
        game_path
    } else {
        ""
    });
}
