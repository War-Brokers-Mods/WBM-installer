use super::util;
use super::InstallErr;

pub async fn clean(game_path: &str) -> Result<(), InstallErr> {
    match util::uninstall(game_path) {
        Ok(_) => {}

        Err(_) => {
            return Err(InstallErr::RemoveOldFilesFailed);
        }
    }

    return Ok(());
}
