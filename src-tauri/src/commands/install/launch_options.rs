use tauri::Window;

use super::InstallResult;
use crate::commands::install::{emit, InstallSteps};
use crate::constants;
use crate::util;

use std::fs::File;
use std::io::{BufRead, BufReader};

/// the launch option
// const LAUNCH_OPTIONS: &str = "./run_bepinex.sh %command%";

/// the actual localconfig.vdf line storing the launch option data
// const LAUNCH_OPTION_LINE: &str = r#"						"LaunchOptions"		"./run_bepinex.sh %command%""#;

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

    // get localconfig.vdf path

    let localconfig_path = match get_localconfig_path() {
        Some(localconfig_path) => localconfig_path,

        None => return Err(InstallResult::LaunchOptionFailed),
    };

    // write launch option to the config file

    let localconfig_file = File::open(localconfig_path).unwrap();
    let localconfig_reader = BufReader::new(localconfig_file);

    // go through each line of the file
    for (index, line) in localconfig_reader.lines().enumerate() {
        let line = match line {
            Ok(line) => line,

            Err(err) => {
                println!("Error while reading line: {:#?}", err);
                continue;
            }
        };

        // get the line containing and only containing the substring "750470" including the quote
        if line.contains(constants::STEAM_APPID) && line.matches("\"").count() == 2 {
            println!("{}| {}", index, line)
        }
    }

    emit(&window, InstallSteps::LaunchOption);
    return Ok(());
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
