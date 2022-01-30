use tauri::api::path::{cache_dir, home_dir};

use futures_util::StreamExt;
use std::cmp::min;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// build client with header
pub fn build_client() -> reqwest::Client {
    return reqwest::Client::builder()
        .user_agent("WBM-Installer")
        .build()
        .unwrap();
}

/// Gets platform-specific default steam path.
pub fn get_default_steam_path() -> Option<String> {
    let home = buf2str(home_dir());
    if home.is_none() {
        println!("Couldn't get home directory");
        return None;
    }
    let home = home.unwrap();

    let steam_path = match std::env::consts::OS {
        "linux" => format!("{}/.steam/steam", home),
        "macos" => format!("{}/Library/Application Support/Steam", home),
        "windows" => String::from("C:\\Program Files (x86)\\Steam"),

        _ => {
            println!("Unsupported OS");
            return None;
        }
    };

    // check if path is valid
    if fs::metadata(steam_path.as_str()).is_err() {
        println!("Default steam path not found");
        return None;
    }

    return Some(String::from(steam_path));
}

/// Gets platform-specific default game path.
pub fn get_default_game_path() -> Option<String> {
    match get_default_steam_path() {
        Some(steam_path) => {
            let game_path = match std::env::consts::OS {
                "linux" | "macos" => format!("{}/steamapps/common/WarBrokers", steam_path),
                "windows" => String::from(format!("{}\\steamapps\\common\\WarBrokers", steam_path)),

                _ => return None,
            };

            if fs::metadata(game_path.as_str()).is_err() {
                println!("Default game path not found");
                return None;
            }

            if !is_game_path_valid(&game_path) {
                return None;
            }

            return Some(String::from(game_path));
        }

        None => return None,
    }
}

/// Checks if the path is a valid War Brokers game path
pub fn is_game_path_valid(_game_path: &str) -> bool {
    // todo: implement logic

    return true;
}

/// convert `Option<PathBuf>` to `Option<String>`
pub fn buf2str(input: Option<PathBuf>) -> Option<String> {
    if input.is_none() {
        return None;
    }

    return Some(String::from(input.unwrap().to_str().unwrap()));
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
pub async fn download_zip_to_cache_dir(url: &str, file_name: &str) -> Result<String, String> {
    // parse path

    let cache_dir_raw = buf2str(cache_dir());
    if cache_dir_raw.is_none() {
        // todo: handle error here
        panic!("Failed to get cache directory.");
    }

    let path = format!("{}/{}", cache_dir_raw.unwrap(), file_name);
    let path_str = path.as_str();

    // initialize reqwest client

    let client = build_client();

    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;

    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    // download chunks

    let mut file =
        File::create(path_str).or(Err(format!("Failed to create file '{}'", path_str)))?;
    let mut stream = res.bytes_stream();

    let mut downloaded: u64 = 0;

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;

        file.write(&chunk)
            .or(Err(format!("Error while writing to file")))?;

        let new = min(downloaded + (chunk.len() as u64), total_size);

        downloaded = new;
    }

    return Ok(path);
}

pub fn unzip(path: &str, destination: &str) -> Result<(), zip::result::ZipError> {
    let fname = std::path::Path::new(path);
    let zipfile = std::fs::File::open(&fname).unwrap();
    let mut archive = zip::ZipArchive::new(zipfile).unwrap();

    return archive.extract(destination);
}

/// Uninstall WBM and related files
pub fn uninstall(_game_path: &str) -> Result<(), ()> {
    // todo: implement

    return Ok(());
}
