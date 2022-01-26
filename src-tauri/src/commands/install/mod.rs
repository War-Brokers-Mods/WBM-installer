// [Sync]: must be synced with `src/pages/Install/index.svelte`
use tauri::Window;

use crate::constants;
use crate::util;

mod launch_options;

use std::env;

// [Sync]
#[derive(Clone)]
pub enum InstallSteps {
    DownloadBepInEx,
    InstallBepInEx,
    LaunchOption,
    LaunchGame,
    DownloadWbmZip,
    InstallWbm,
    Done,
}

// [Sync]
pub enum InstallResult {
    NoErr,
    FailedToGetGamePath,
    UnsupportedOS,
    BepInExDownloadFailed,
    BepInExUnzipFailed,
    WBMDownloadFailed,
    WBMRemoveFailed,
    WBMDirectoryCreationFailed,
    WBMUnzipFailed,
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
    // Get game path
    //

    let game_path = match get_game_path(game_path) {
        Ok(game_path) => game_path,

        Err(err) => return err as i64,
    };
    let game_path = game_path.as_str();

    //
    // Install BepInEx
    //

    match install_bepinex(&window, game_path).await {
        Ok(()) => {}
        Err(err) => return err as i64,
    }

    //
    // Setup steam launch option if OS is linux or macOS
    //

    match launch_options::unix_launch_option_setup(&window).await {
        Ok(()) => {}
        Err(err) => return err as i64,
    }

    //
    // Run the game once to generate the plugins directory
    //

    match launch_game_once(&window).await {
        Ok(()) => {}
        Err(err) => return err as i64,
    }

    //
    // Install the mod
    //

    match install_wbm_mod(&window, game_path).await {
        Ok(()) => {}
        Err(err) => return err as i64,
    }

    //
    // Tell the frontend that the installation was successful
    //

    emit(&window, InstallSteps::Done);
    println!("Install complete!");

    return InstallResult::NoErr as i64;
}

/// Get the absolute path to the game install directory
fn get_game_path(game_path: String) -> Result<String, InstallResult> {
    let game_path = if game_path.is_empty() {
        // if game_path argument is empty, get the default path

        let default_game_path = util::get_default_game_path();
        if default_game_path.is_none() {
            // failed to find game install location.
            // Prompt user to manually choose the game location.
            return Err(InstallResult::FailedToGetGamePath);
        }
        default_game_path.unwrap()
    } else {
        // otherwise, use the passed value

        // todo: check if game path is valid
        game_path
    };

    return Ok(game_path);
}

async fn install_bepinex(window: &Window, game_path: &str) -> Result<(), InstallResult> {
    println!();
    println!("Installing BepInEx");

    // determine which BepInEx file to download
    // download URL is updated manually
    // latest release files can be found here: https://github.com/BepInEx/BepInEx/releases
    let bepinex_zip_url = match env::consts::OS {
        "linux" | "macos" => {
            "https://github.com/BepInEx/BepInEx/releases/download/v5.4.18/BepInEx_unix_5.4.18.0.zip"
        }

        "windows" => {
            "https://github.com/BepInEx/BepInEx/releases/download/v5.4.18/BepInEx_x86_5.4.18.0.zip"
        }

        _ => {
            println!("Unsupported OS!");
            return Err(InstallResult::UnsupportedOS);
        }
    };

    // download file to cache directory

    println!("Downloading BepInEx.zip");
    match util::download_zip_to_cache_dir(bepinex_zip_url, "BepInEx.zip").await {
        Ok(bepinex_path) => {
            println!("Downloaded BepInEx.zip to '{}'", bepinex_path);
            println!("Unzipping BepInEx.zip");

            match util::unzip(bepinex_path.as_str(), &game_path) {
                Ok(()) => {
                    emit(&window, InstallSteps::InstallBepInEx);
                    println!("Successfully unzipped BepInEx.zip to {}", game_path);
                }

                Err(err) => {
                    println!("Failed to unzip BepInEx.zip ({:#?})", err);
                    return Err(InstallResult::BepInExUnzipFailed);
                }
            }
        }

        Err(_) => {
            println!("Failed to download BepInEx.zip");
            return Err(InstallResult::BepInExDownloadFailed);
        }
    }

    // done

    emit(&window, InstallSteps::DownloadBepInEx);
    return Ok(());
}

async fn launch_game_once(window: &Window) -> Result<(), InstallResult> {
    println!();
    println!("Launch Game once");

    emit(&window, InstallSteps::LaunchGame);
    return Ok(());
}

async fn install_wbm_mod(window: &Window, game_path: &str) -> Result<(), InstallResult> {
    println!();
    println!("Installing WBM mod");

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
                    println!("Failed to remove existing WBM mod files");
                    return Err(InstallResult::WBMRemoveFailed);
                }
            };

            println!("Creating WBM directory");
            match std::fs::create_dir_all(wbm_path.clone()) {
                Ok(_) => {}
                Err(_) => {
                    println!("Failed to create WBM mod directory");
                    return Err(InstallResult::WBMDirectoryCreationFailed);
                }
            }

            // unzip file
            match util::unzip(zip_path.as_str(), wbm_path.to_str().unwrap()) {
                Ok(()) => {
                    emit(&window, InstallSteps::InstallWbm);
                }

                Err(err) => {
                    println!("Failed to unzip WBM.zip: ({:#?})", err);
                    return Err(InstallResult::WBMUnzipFailed);
                }
            };
        }

        Err(_) => {
            println!("Failed to download WBM.zip");
            return Err(InstallResult::WBMDownloadFailed);
        }
    }

    // done

    emit(&window, InstallSteps::DownloadWbmZip);
    return Ok(());
}

//
//
//

pub fn emit(window: &Window, payload: InstallSteps) {
    util::emit(&window, constants::EVENT_INSTALL, payload as i64);
}
