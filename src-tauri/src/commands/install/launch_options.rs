use super::InstallErr;

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
