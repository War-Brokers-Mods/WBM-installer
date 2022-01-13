use std::fs;

/// build client with header
pub fn build_client() -> reqwest::Client {
    return reqwest::Client::builder()
        .user_agent("WBM-Installer")
        .build()
        .unwrap();
}

/// gets path to WB game files.
pub fn get_default_game_path() -> Result<String, String> {
    let game_path = match std::env::consts::OS {
        "linux" => "~/.steam/steam/steamapps/common/WarBrokers",
        "macos" => "~/Library/Application Support/Steam/steamapps/common/WarBrokers",
        "windows" => "C:\\Program Files\\Steam\\steamapps\\common\\WarBrokers",

        _ => panic!("Unsupported platform!"),
    };

    // https://stackoverflow.com/a/32384768/12979111
    return if fs::metadata(game_path).is_ok() {
        Ok(String::from(game_path))
    } else {
        Err(String::from("Failed to default game path"))
    };
}
