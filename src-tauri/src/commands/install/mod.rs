// [Sync]: must be synced with `src/pages/Install/types.ts`
use tauri::Window;

use crate::constants;
use crate::util;

mod install_bepinex;
mod install_mod;
mod launch_game;
mod launch_options;

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
    SetLaunchOption,
    LaunchGame,
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

    //
    // Test if OS is compatible
    //

    match std::env::consts::OS {
        "linux" | "macos" | "windows" => {}

        _ => {
            return InstallResult::UnsupportedOS as i64;
        }
    }

    //
    // Get game path
    //

    let game_path = match util::get_game_path(game_path) {
        Some(game_path) => game_path,

        None => return InstallResult::FailedToGetGamePath as i64,
    };
    let game_path = game_path.as_str();

    //
    // Install BepInEx
    //

    match install_bepinex::install_bepinex(&window, game_path).await {
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

    match launch_game::launch_game_once(&window).await {
        Ok(()) => {}
        Err(err) => return err as i64,
    }

    //
    // Install the mod
    //

    match install_mod::install_wbm_mod(&window, game_path).await {
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

pub fn emit(window: &Window, payload: InstallSteps) {
    util::emit(&window, constants::EVENT_INSTALL, payload as i64);
}
