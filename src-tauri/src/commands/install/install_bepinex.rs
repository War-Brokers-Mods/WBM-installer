use std::env;

use tauri::Window;

use super::InstallResult;
use crate::commands::install::{emit, InstallSteps};
use crate::util;

pub async fn install_bepinex(window: &Window, game_path: &str) -> Result<(), InstallResult> {
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
