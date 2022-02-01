use super::InstallErr;
use crate::util;

use std::fs;
use std::os::unix::prelude::PermissionsExt;
use std::path::Path;

pub async fn unix_launch_option_setup(
    window: &tauri::Window,
    game_path: &str,
) -> Result<(), InstallErr> {
    //
    // skip if the OS is not linux or macOS
    //

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

    //
    // Skip if launch option is already set
    //

    if is_already_set() {
        println!("Steam launch option is already set. Skipping.");
        return Ok(());
    }

    //
    // make run_bepinex.sh executable
    //

    let run_bepinex_sh_path = Path::new(game_path).join("run_bepinex.sh");
    let run_bepinex_sh_path_str = run_bepinex_sh_path.to_str().unwrap();
    let perms = fs::metadata(&run_bepinex_sh_path);
    if perms.is_err() {
        println!("Failed to make {} executable", run_bepinex_sh_path_str);
        return Err(InstallErr::LaunchOptionNotSet);
    }
    let mut perms = perms.unwrap().permissions();
    perms.set_mode(0o755); // rwxr-xr-x
    match fs::set_permissions("path", perms) {
        Err(_) => {
            println!("Failed to make {} executable", run_bepinex_sh_path_str);
        }

        _ => {}
    };

    //
    // build launch option string
    //

    let launch_option_str = match std::env::consts::OS {
        "linux" => String::from("./run_bepinex.sh %command%"),

        "macos" => format!("\"{}\" %command%", run_bepinex_sh_path_str),

        _ => {
            return Err(InstallErr::UnsupportedOS);
        }
    };

    //
    // send launch option string to frontend
    //

    match window.emit("launch-option-string", launch_option_str) {
        Err(_) => {
            println!("Failed to send launch option data to the frontend");
            return Err(InstallErr::FailedToSendLaunchOption);
        }

        _ => {}
    };

    println!("Steam launch option is either not set or invalid.");
    println!("Prompting to set launch option.");
    return Err(InstallErr::LaunchOptionNotSet);
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
        Ok(_content) => {
            // todo: improve logic

            // 1. find line only containing "750470"
            // 2. find next closest line only containing "}"
            // 3. check if section has line containing both "run_bepinex.sh" and "%command%"

            return true;
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
