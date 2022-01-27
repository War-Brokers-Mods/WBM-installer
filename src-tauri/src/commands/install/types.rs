/// must be synced with `src/pages/Install/types.ts`

//
//
//

#[derive(Clone, Copy)]
pub enum InstallSteps {
    DownloadBepInEx,
    InstallBepInEx,
    LaunchOption,
    LaunchGame,
    DownloadWbmZip,
    InstallWbm,
    Done,
}

impl serde::Serialize for InstallSteps {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(*self as i64)
    }
}

//
//
//

#[derive(Clone, Copy, PartialEq)]
pub enum InstallResult {
    NoErr,
    SetLaunchOption,
    LaunchGame,
    Skip, // only used for subcommands
}

impl serde::Serialize for InstallResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(*self as i64)
    }
}

//
//
//

#[derive(Clone, Copy)]
pub enum InstallErr {
    FailedToGetGamePath,
    UnsupportedOS,
    BepInExDownloadFailed,
    BepInExUnzipFailed,
    WBMDownloadFailed,
    WBMRemoveFailed,
    WBMDirectoryCreationFailed,
    WBMUnzipFailed,
}

impl serde::Serialize for InstallErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(*self as i64)
    }
}

// #[derive(Clone, serde::Serialize)]
// struct InstallPayload(i64);
