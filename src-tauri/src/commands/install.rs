use tauri::Window;

use crate::constants;
use crate::util;

use std::env;

// must be synced with `src/pages/Install/index.svelte`
#[derive(Clone)]
enum InstallSteps {
    DownloadBepInEx,
    DownloadWbmZip,
    Done,
}

#[derive(Clone, serde::Serialize)]
struct InstallPayload(i64);

// todo: show current step in the frontend

/// automated version of https://github.com/War-Brokers-Mods/WBM#installation
#[tauri::command]
pub async fn install(window: Window) {
    println!("Installing WBM");

    // todo: download things in parallel when possible

    // get game path

    let game_path = util::get_default_game_path();
    if game_path.is_none() {
        // todo: handle error
        panic!("failed to find game install location");
    }
    let game_path = game_path.unwrap();
    let game_path = game_path.as_str();

    // start installation

    install_bepinex(&window, game_path).await;
    download_wbm_zip(&window, game_path).await;

    util::emit(&window, constants::EVENT_INSTALL, InstallSteps::Done as i64);
    println!("Install complete!");
}

async fn install_bepinex(window: &Window, game_path: &str) {
    println!("Installing BepInEx");

    // determine which BepInEx file to download

    // download URL is updated manually
    let bepinex_zip_url = match env::consts::OS {
        "linux" | "macos" => {
            "https://github.com/BepInEx/BepInEx/releases/download/v5.4.18/BepInEx_unix_5.4.18.0.zip"
        }

        "windows" => {
            "https://github.com/BepInEx/BepInEx/releases/download/v5.4.18/BepInEx_x86_5.4.18.0.zip"
        }

        // todo: provide user feedback instead of terminating
        _ => {
            panic!("Unsupported OS!");
        }
    };

    // download file to cache directory

    let result = util::download_zip_to_cache_dir(bepinex_zip_url, "BepInEx.zip").await;
    match result {
        Ok(path) => {
            println!("downloaded BepInEx.zip: {}", path);
        }

        Err(_) => {
            // todo: handle error
            panic!("failed to download BepInEx.zip")
        }
    }

    // unzip file

    println!("{}", game_path);

    // done

    util::emit(
        &window,
        constants::EVENT_INSTALL,
        InstallSteps::DownloadBepInEx as i64,
    );
}

async fn download_wbm_zip(window: &Window, game_path: &str) {
    // todo: get url of latest zip

    let latest_release = &json::parse(util::get_wbm_release_data().await.as_str()).unwrap()[0]
        ["assets"][0]["browser_download_url"];

    let download_zip_result =
        util::download_zip_to_cache_dir(latest_release.as_str().unwrap(), "WBM.zip").await;

    match download_zip_result {
        Ok(path) => {
            println!("downloaded WBM.zip: {}", path)
        }

        Err(_) => {
            // todo: handle error
            panic!("Failed to download WBM.zip");
        }
    }

    // unzip file

    println!("{}", game_path);

    // done

    util::emit(
        &window,
        constants::EVENT_INSTALL,
        InstallSteps::DownloadWbmZip as i64,
    );
}
