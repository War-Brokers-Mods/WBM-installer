// [SyncConst]: must be synced with `src/constants.ts`
// [SyncStatus]: must be synced with `src/pages/Status/index.svelte`
use crate::util;

/// request types.
/// [SyncConst]
static LATEST_VERSION: &'static str = "LATEST_VERSION";
static GAME_PATH: &'static str = "GAME_PATH";

#[tauri::command]
pub async fn status(req_type: String) -> StatusData {
    let mut res = StatusData::default();

    if req_type == LATEST_VERSION {
        res.latest_release_version = get_latest_release().await;
    } else if req_type == GAME_PATH {
    }

    return res.clone();
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

/// [SyncStatus]
#[derive(serde::Serialize, Default, Clone)]
pub struct StatusData {
    latest_release_version: String,
    game_path: String,
}
