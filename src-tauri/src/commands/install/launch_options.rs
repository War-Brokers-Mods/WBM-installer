use tauri::Window;

use super::InstallResult;
use crate::commands::install::{emit, InstallSteps};

pub async fn unix_launch_option_setup(window: &Window) -> Result<(), InstallResult> {
    // skip if the OS is not linux or macOS
    match std::env::consts::OS {
        "linux" | "macos" => {
            println!();
            println!("Setting up steam launch option");
        }

        _ => {
            println!();
            println!("Skipping unix launch option setup");

            return Ok(());
        }
    };

    println!("Prompt user to launch option");

    emit(&window, InstallSteps::LaunchOption);
    return Ok(());
}
