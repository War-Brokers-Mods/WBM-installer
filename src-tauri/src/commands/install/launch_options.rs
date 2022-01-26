use tauri::Window;

use super::InstallResult;
use crate::commands::install::{emit, InstallSteps};
use crate::util;

use std::fs;

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

    if is_already_set() {
        println!("Steam launch option is already set. Skipping.");
        return Ok(());
    }

    println!("Prompt user to launch option");
    emit(&window, InstallSteps::LaunchOption);
    return Err(InstallResult::SetLaunchOption); // stop install function
}

fn is_already_set() -> bool {
    //
    // get localconfig.vdf path
    //

    let localconfig_path = match get_localconfig_path() {
        Some(localconfig_path) => localconfig_path,

        None => {
            println!("Failed to locate localconfig.vdf");
            return false;
        }
    };

    //
    // check if launch option is set
    //

    match fs::read_to_string(localconfig_path) {
        Ok(content) => {
            return content.contains("./run_bepinex.sh %command%");
        }

        Err(err) => {
            println!("Failed to read localconfig.vdf: {:#?}", err);
            return false;
        }
    };
}

fn get_localconfig_path() -> Option<String> {
    match util::get_default_steam_path() {
        // using forward slash because this part of the code is only reachable in linux and macOS
        Some(steam_path) => {
            // get steam user ID

            let paths = std::fs::read_dir(format!("{}/userdata", steam_path)).unwrap();
            let mut user_path: &str = "";
            let mut _user_path_buf: std::path::PathBuf;

            let mut count = 0;
            for path in paths {
                count += 1;

                _user_path_buf = path.unwrap().path();
                user_path = _user_path_buf.to_str().unwrap();
            }

            if count > 1 {
                // todo: choose steam user ID when there's multiple
                println!("Failed to get steam user");
                return None;
            }

            return Some(format!("{}/config/localconfig.vdf", user_path));
        }

        None => return None,
    };
}
