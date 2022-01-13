// [Sync]: must be synced with `src/pages/Status/index.svelte`

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
        // returns an empty string if the game doesn't exist in the default path
        match util::get_default_game_path() {
            Ok(path) => status_data.game_path = path,
            Err(_) => {}
        }
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
