/// must be synced with `src/pages/Install/types.ts`

#[derive(Clone, Copy)]
pub enum InstallErr {
    UnsupportedOS,
    FailedToGetGamePath,
    GamePathNotValid,
    RemoveOldFilesFailed,
    BepInExDownloadFailed,
    BepInExUnzipFailed,
    WBMDownloadFailed,
    WBMRemoveFailed,
    WBMDirectoryCreationFailed,
    WBMUnzipFailed,
    LaunchOptionNotSet,
}

impl serde::Serialize for InstallErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(*self as i64)
    }
}
