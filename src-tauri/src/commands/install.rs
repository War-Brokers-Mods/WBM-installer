use tauri::api::path::cache_dir;

use crate::util;

#[tauri::command]
pub async fn install() {
    let game_path = util::get_default_game_path();
    if game_path.is_some() {
        println!("game path: {}", game_path.unwrap());
    }

    let cache_dir_raw = util::buf2str(cache_dir());
    if cache_dir_raw.is_some() {
        let result = util::download_release_zip(
            &"https://github.com/War-Brokers-Mods/WBM/releases/download/v1.7.1.0/WBM.zip",
            format!("{}/WBM.zip", cache_dir_raw.unwrap()).as_str(),
        )
        .await;

        println!("{:#?}", result);
    }
}
