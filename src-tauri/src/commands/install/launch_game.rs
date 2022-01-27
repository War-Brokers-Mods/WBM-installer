use tauri::Window;

use super::{InstallErr, InstallResult};
use crate::commands::install::{emit, InstallSteps};

pub async fn launch_game_once(window: &Window) -> Result<InstallResult, InstallErr> {
    println!();
    println!("Launch Game once");

    emit(&window, InstallSteps::LaunchGame);
    return Ok(InstallResult::LaunchGame); // stop install
}
