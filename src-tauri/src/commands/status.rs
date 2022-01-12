use crate::util;

#[tauri::command]
pub async fn status() {
    println!("returning status!");
    ping().await;
}

/// ping github and get latest version at the same time
async fn ping() -> bool {
    let client = util::build_client();

    // get the latest version of WBM release
    // todo: parse json
    let res = client
        .get("https://api.github.com/repos/War-Brokers-Mods/WBM/releases")
        .send()
        .await
        .unwrap();

    println!("{:#?}", res);

    // return true if status code is 200
    return res.status() == 200;
}
