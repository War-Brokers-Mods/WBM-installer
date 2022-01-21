// [Sync]: must be synced with `src/pages/Install/index.svelte`
use tauri::Window;

use crate::constants;
use crate::util;

use std::env;

// [Sync]
#[derive(Clone)]
enum InstallSteps {
    DownloadBepInEx,
    InstallBepInEx,
    UnixLaunchOption,
    LaunchGame,
    DownloadWbmZip,
    InstallWbm,
    Done,
}

// [Sync]
enum InstallResult {
    NoErr,
    FailedToGetGamePath,
}

#[derive(Clone, serde::Serialize)]
struct InstallPayload(i64);

// todo: show current step in the frontend

/// automated version of https://github.com/War-Brokers-Mods/WBM#installation
#[tauri::command]
pub async fn install(window: Window, game_path: String) -> i64 {
    println!("Installing WBM");

    // todo: download things in parallel when possible

    //
    // get game path
    //

    let game_path = if game_path.is_empty() {
        // if game_path argument is empty, get the default path

        let default_game_path = util::get_default_game_path();
        if default_game_path.is_none() {
            // failed to find game install location.
            // Prompt user to manually choose the game location.
            return InstallResult::FailedToGetGamePath as i64;
        }
        default_game_path.unwrap()
    } else {
        // otherwise, use the passed value

        // todo: check if game path is valid
        game_path
    };
    let game_path = game_path.as_str();

    //
    // start installation
    //

    install_bepinex(&window, game_path).await;
    unix_launch_option_setup(&window, game_path).await;
    launch_game_once(&window).await;
    install_wbm_mod(&window, game_path).await;

    //
    //
    //

    emit(&window, InstallSteps::Done);
    println!("Install complete!");

    return InstallResult::NoErr as i64;
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

    println!("Downloading BepInEx.zip");
    let result = util::download_zip_to_cache_dir(bepinex_zip_url, "BepInEx.zip").await;
    match result {
        Ok(bepinex_path) => {
            println!("Downloaded BepInEx.zip: {}", bepinex_path);
            println!("Unzipping BepInEx.zip");
            match util::unzip(bepinex_path.as_str(), &game_path) {
                Ok(()) => {
                    emit(&window, InstallSteps::InstallBepInEx);
                    println!("Successfully unzipped BepInEx.zip to {}", game_path);
                }

                Err(err) => {
                    panic!("{:#?}", err);
                }
            }
        }

        Err(_) => {
            // todo: handle error
            panic!("Failed to download BepInEx.zip")
        }
    }

    // done

    emit(&window, InstallSteps::DownloadBepInEx);
}

async fn unix_launch_option_setup(window: &Window, game_path: &str) {
    println!("{}", game_path);

    emit(&window, InstallSteps::UnixLaunchOption);
}

async fn launch_game_once(window: &Window) {
    println!("Launch Game once");
    emit(&window, InstallSteps::LaunchGame);
}

async fn install_wbm_mod(window: &Window, game_path: &str) {
    // todo: get url of latest zip

    let latest_release = &json::parse(util::get_wbm_release_data().await.as_str()).unwrap()[0]
        ["assets"][0]["browser_download_url"];

    let download_zip_result =
        util::download_zip_to_cache_dir(latest_release.as_str().unwrap(), "WBM.zip").await;

    match download_zip_result {
        Ok(zip_path) => {
            let wbm_path = std::path::Path::new(game_path).join("BepInEx/plugins/WBM");

            println!("Removing existing files");
            match std::fs::remove_dir_all(wbm_path.clone()) {
                Ok(_) => {}
                Err(_) => {
                    panic!("Failed to remove existing WBM mod files");
                }
            };

            println!("Creating WBM directory");
            match std::fs::create_dir_all(wbm_path.clone()) {
                Ok(_) => {}
                Err(_) => {
                    panic!("Failed to create WBM mod directory");
                }
            }

            // unzip file
            // todo: handle unwrap error
            match util::unzip(zip_path.as_str(), wbm_path.to_str().unwrap()) {
                Ok(()) => {
                    emit(&window, InstallSteps::InstallWbm);
                }

                Err(err) => {
                    // todo: handle error
                    panic!("Failed to unzip WBM.zip:{:#?}", err);
                }
            };
        }

        Err(_) => {
            // todo: handle error
            panic!("Failed to download WBM.zip");
        }
    }

    // done

    emit(&window, InstallSteps::DownloadWbmZip);
}

fn emit(window: &Window, payload: InstallSteps) {
    util::emit(&window, constants::EVENT_INSTALL, payload as i64);
}
