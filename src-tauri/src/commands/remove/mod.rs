mod types;

use types::RemoveErr;

use crate::util;

#[tauri::command]
pub async fn remove(game_path: String) -> Result<(), RemoveErr> {
    println!("Remove command called");

    //
    // Test if OS is compatible
    //

    match std::env::consts::OS {
        "linux" | "macos" | "windows" => {}

        _ => {
            println!("Unsupported OS!");
            return Err(RemoveErr::UnsupportedOS);
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
            None => return Err(RemoveErr::FailedToGetGamePath),
        };

        default_game_path
    } else {
        game_path
    };
    let game_path = game_path.as_str();

    //
    // Remove files
    //

    match util::uninstall(&game_path) {
        Ok(_) => {}

        Err(_) => {
            return Err(RemoveErr::FailedToRemoveFiles);
        }
    }

    //
    // todo: Reset steam launch option
    //

    return Ok(());
}
