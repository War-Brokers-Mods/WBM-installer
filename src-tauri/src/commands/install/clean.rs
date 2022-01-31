use super::util;
use super::InstallErr;

pub async fn clean(game_path: &str) -> Result<(), InstallErr> {
    match util::uninstall(game_path) {
        Ok(_) => {
            return Ok(());
        }

        Err(_) => {
            return Err(InstallErr::RemoveOldFilesFailed);
        }
    };
}
