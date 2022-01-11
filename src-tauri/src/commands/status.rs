#[tauri::command]
pub fn status() {
    println!("returning status!")
}
