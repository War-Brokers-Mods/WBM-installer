use tauri::Window;

use crate::constants;
use crate::util;

mod types;

mod install_bepinex;
mod install_mod;
mod launch_game;
mod launch_options;

use types::{InstallErr, InstallResult, InstallSteps};

// todo: show current step in the frontend

/// automated version of the [manual installation](https://github.com/War-Brokers-Mods/WBM#installation).
///
/// This function exits if it requires a user input and is called again with the user feedback as its arguments.
///
/// ## Installation procedure
///
/// This function exits at the end of each step.
///
/// 1. BepInEx installation
/// 2. Steam launch option setup (only on Linux and MacOS)
/// 3. Launch game for plugins folder generation
/// 4. Mod installation
///
/// Some part of the function are unnecessary executed each time the function is called,
/// but the time loss is negligible and it's a sacrifice worth for code readability.
///
/// ## Arguments
///
/// All arguments except `windows` are empty by default.
///
/// * `window` - standard tauri argument. See [docs](https://tauri.studio/docs/guides/command#accessing-the-window-in-commands) for more info.
/// * `game_path` - absolute path to the game folder/directory.
/// * `was_game_launched` - whether if the game was launched once after installing BepInEx to generate the plugins folder.
#[tauri::command]
pub async fn install(
    window: Window,
    game_path: String,
    was_game_launched: bool,
) -> Result<InstallResult, InstallErr> {
    println!("install command called");

    //
    // Test if OS is compatible
    //

    match std::env::consts::OS {
        "linux" | "macos" | "windows" => {}

        _ => {
            println!("Unsupported OS!");
            return Err(InstallErr::UnsupportedOS);
        }
    }

    //
    // Resolve game path
    //

    let game_path = if game_path.is_empty() {
        let default_game_path = match util::get_default_game_path() {
            Some(path) => path,

            // failed to find game install location.
            // Prompt user to manually choose the game location.
            None => return Err(InstallErr::FailedToGetGamePath),
        };

        default_game_path
    } else {
        // todo: check if game path is valid and tell the user
        game_path
    };
    let game_path = game_path.as_str();

    //
    // Install BepInEx
    //

    match install_bepinex::install_bepinex(&window, game_path).await {
        Ok(()) => {}
        Err(err) => return Err(err),
    }

    //
    // Setup steam launch option if OS is linux or macOS
    //

    match launch_options::unix_launch_option_setup(&window).await {
        Ok(_) => {}
        Err(err) => return Err(err),
    }

    //
    // Run the game once to generate the plugins directory
    //

    match launch_game::launch_game_once(&window, was_game_launched).await {
        Ok(res) => {
            if res != InstallResult::Skip {
                return Ok(res);
            }
        }
        Err(err) => return Err(err),
    }

    //
    // Install the mod
    //

    match install_mod::install_wbm_mod(&window, game_path).await {
        Ok(()) => {}
        Err(err) => return Err(err),
    }

    //
    // Tell the frontend that the installation was successful
    //

    emit(&window, InstallSteps::Done);
    println!("Install complete!");

    return Ok(InstallResult::NoErr);
}

pub fn emit(window: &Window, payload: InstallSteps) {
    util::emit(&window, constants::EVENT_INSTALL, payload);
}
