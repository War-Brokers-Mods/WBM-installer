mod types;

use types::RemoveErr;

#[tauri::command]
pub async fn remove(_game_path: String) -> Result<(), RemoveErr> {
    return Ok(());
}
