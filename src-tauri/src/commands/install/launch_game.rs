use tauri::Window;

use super::InstallResult;
use crate::commands::install::{emit, InstallSteps};

pub async fn launch_game_once(window: &Window) -> Result<(), InstallResult> {
    println!();
    println!("Launch Game once");

    emit(&window, InstallSteps::LaunchGame);
    return Ok(());
}
