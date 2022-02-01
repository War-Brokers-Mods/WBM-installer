use tauri::api::path::{cache_dir, home_dir};

use futures_util::StreamExt;
use std::io::Write;

use std::fs::{metadata, remove_dir_all, remove_file, File};
use std::path::Path;

/// build new reqwest client with custom header
pub fn build_client() -> reqwest::Client {
    return reqwest::Client::builder()
        .user_agent("WBM-Installer")
        .build()
        .unwrap();
}

/// Gets absolute path to the default steam directory.
pub fn get_default_steam_path() -> Option<String> {
    let home = home_dir();
    if home.is_none() {
        println!("Couldn't get home directory");
        return None;
    }
    let home = home.unwrap();

    let steam_path = match std::env::consts::OS {
        "linux" => String::from(Path::new(&home).join(".steam/steam").to_str().unwrap()),
        "macos" => String::from(
            Path::new(&home)
                .join("Library/Application Support/Steam")
                .to_str()
                .unwrap(),
        ),
        "windows" => String::from("C:\\Program Files (x86)\\Steam"),

        _ => {
            println!("Unsupported OS");
            return None;
        }
    };

    // check if path is valid
    if metadata(steam_path.as_str()).is_err() {
        println!("Default steam path not found");
        return None;
    }

    return Some(steam_path);
}

/// Gets absolute path to the default game directory.
pub fn get_default_game_path() -> Option<String> {
    match get_default_steam_path() {
        Some(steam_path) => {
            let game_path = match std::env::consts::OS {
                "linux" | "macos" => String::from(
                    Path::new(&steam_path)
                        .join("steamapps/common/WarBrokers")
                        .to_str()
                        .unwrap(),
                ),
                "windows" => String::from(
                    Path::new(&steam_path)
                        .join("steamapps\\common\\WarBrokers")
                        .to_str()
                        .unwrap(),
                ),

                _ => return None,
            };

            if metadata(game_path.as_str()).is_err() {
                println!("Default game path not found");
                return None;
            }

            if !is_game_path_valid(&game_path) {
                return None;
            }

            return Some(String::from(game_path));
        }

        None => return None,
    };
}

/// Checks if the path is a valid War Brokers game path
pub fn is_game_path_valid(_game_path: &str) -> bool {
    // todo: implement

    return true;
}

/// get the latest WBM release version.
/// data is not converted to a json object because it'll be done in the front-end
pub async fn get_wbm_release_data() -> String {
    let client = build_client();

    // todo: handle error
    let res = client
        .get("https://api.github.com/repos/War-Brokers-Mods/WBM/releases")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    return res;
}

/// downloads a file from the url to the cache directory.
pub async fn download_zip_to_cache_dir(url: &str, file_name: &str) -> Result<String, ()> {
    // parse path

    let cache_dir_path = cache_dir();
    let cache_dir_path = if cache_dir_path.is_none() {
        println!("Failed to get cache directory.");
        return Err(());
    } else {
        String::from(cache_dir_path.unwrap().to_str().unwrap())
    };

    let cache_dir_path = Path::new(&cache_dir_path).join(file_name);
    let cache_dir_path = cache_dir_path.to_str().unwrap();

    // initialize reqwest client

    let client = build_client();

    let res = client.get(url).send().await;
    let res = if res.is_err() {
        println!("Failed to GET from '{}'", &url);
        return Err(());
    } else {
        res.unwrap()
    };

    let total_size = res.content_length();
    let total_size = if total_size.is_none() {
        println!("Failed to get content length from: '{}'", url);
        return Err(());
    } else {
        total_size.unwrap()
    };

    // download chunks

    let file = File::create(cache_dir_path);
    let mut file = if file.is_err() {
        println!("Failed to create file '{}'", cache_dir_path);
        return Err(());
    } else {
        file.unwrap()
    };

    let mut stream = res.bytes_stream();

    let mut downloaded: u64 = 0;

    while let Some(item) = stream.next().await {
        let chunk = if item.is_err() {
            println!("Error while downloading file");
            return Err(());
        } else {
            item.unwrap()
        };

        if file.write(&chunk).is_err() {
            println!("Error while writing to file");
            return Err(());
        }

        let new = std::cmp::min(downloaded + (chunk.len() as u64), total_size);

        downloaded = new;
    }

    return Ok(String::from(cache_dir_path));
}

pub fn unzip(path: &str, destination: &str) -> Result<(), zip::result::ZipError> {
    let fname = Path::new(path);
    let zipfile = File::open(&fname).unwrap();
    let mut archive = zip::ZipArchive::new(zipfile).unwrap();

    return archive.extract(destination);
}

/// Partially uninstalls WBM and related files.
///
/// Be careful when running this function! Make sure that the `game_path` is a valid path!
///
/// This function only remove files related to WBM.
/// Removing steam launch option (only applies to Linux and macOS) should be done manually.
pub fn uninstall(game_path: &str) -> Result<(), ()> {
    let game_path = game_path.to_string();

    match std::env::consts::OS {
        "linux" | "macos" => {
            match remove_dir_all(Path::new(&game_path).join("BepInEx")) {
                _ => {}
            };
            match remove_dir_all(Path::new(&game_path).join("doorstop_libs")) {
                _ => {}
            };
            match remove_file(Path::new(&game_path).join("changelog.txt")) {
                _ => {}
            };
            match remove_file(Path::new(&game_path).join("run_bepinex.sh")) {
                _ => {}
            };
        }

        "windows" => {
            match remove_dir_all(Path::new(&game_path).join("BepInEx")) {
                _ => {}
            };
            match remove_file(Path::new(&game_path).join("doorstop_config.ini")) {
                _ => {}
            };
            match remove_file(Path::new(&game_path).join("winhttp.dll")) {
                _ => {}
            };
            match remove_file(Path::new(&game_path).join("changelog.txt")) {
                _ => {}
            };
        }

        _ => {
            return Err(());
        }
    }

    return Ok(());
}
