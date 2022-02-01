use crate::util;

mod types;

mod clean;
mod install_bepinex;
mod install_mod;
mod launch_options;

use types::InstallErr;

// todo: show current step in the frontend

/// automated version of the [manual installation](https://github.com/War-Brokers-Mods/WBM#installation).
///
/// This function exits if it requires a user input and is called again with the user input as its arguments.
///
/// Some parts of the function are unnecessary executed each time the function is called,
/// but the time loss is negligible and it's a sacrifice worth for code readability.
///
/// ## Arguments
///
/// * `game_path` - absolute path to the game folder/directory.
#[tauri::command]
pub async fn install(window: tauri::Window, game_path: String) -> Result<(), InstallErr> {
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
        game_path
    };
    let game_path = game_path.as_str();

    //
    // Remove existing files
    //

    match clean::clean(game_path).await {
        Ok(_) => {}
        Err(err) => return Err(err),
    }

    //
    // Install BepInEx
    //

    match install_bepinex::install_bepinex(game_path).await {
        Ok(()) => {}
        Err(err) => return Err(err),
    }

    //
    // Install the mod
    //

    match install_mod::install_wbm_mod(game_path).await {
        Ok(()) => {}
        Err(err) => return Err(err),
    }

    //
    // Set steam launch option if OS is linux or macOS
    //

    match launch_options::unix_launch_option_setup(&window, game_path).await {
        Ok(_) => {}
        Err(err) => return Err(err),
    }

    //
    // Tell the frontend that the installation was successful
    //

    println!("Install complete!");

    return Ok(());
}
