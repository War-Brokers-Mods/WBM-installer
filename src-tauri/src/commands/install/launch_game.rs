use tauri::Window;

use super::{InstallErr, InstallResult};
use crate::commands::install::{emit, InstallSteps};

pub async fn launch_game_once(
    window: &Window,
    was_game_launched: bool,
) -> Result<InstallResult, InstallErr> {
    println!();
    println!("Launch Game once");

    if was_game_launched {
        return Ok(InstallResult::Skip);
    }

    emit(&window, InstallSteps::LaunchGame);
    return Ok(InstallResult::LaunchGame); // stop install
}
