use tauri::Window;

use super::InstallResult;
use crate::commands::install::{emit, util, InstallSteps};

pub async fn install_wbm_mod(window: &Window, game_path: &str) -> Result<(), InstallResult> {
    println!();
    println!("Installing WBM mod");

    // get the download url for the latest release
    let latest_release = json::parse(util::get_wbm_release_data().await.as_str());
    let latest_release = match &latest_release {
        Ok(data) => match data[0]["assets"][0]["browser_download_url"].as_str() {
            Some(value) => value,

            None => {
                println!("Failed to parse latest release");
                return Err(InstallResult::WBMDownloadFailed);
            }
        },

        Err(_) => {
            println!("Failed to get latest release");
            return Err(InstallResult::WBMDownloadFailed);
        }
    };

    // download zip file
    match util::download_zip_to_cache_dir(latest_release, "WBM.zip").await {
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
