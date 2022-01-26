// types of event
export enum InstallSteps {
	DownloadBepInEx,
	InstallBepInEx,
	LaunchOption,
	LaunchGame,
	DownloadWbmZip,
	InstallWbm,
	Done,
}

// types of install command return codes
export enum InstallResult {
	NoErr,
	FailedToGetGamePath,
	UnsupportedOS,
	BepInExDownloadFailed,
	BepInExUnzipFailed,
	SetLaunchOption,
	LaunchGame,
	WBMDownloadFailed,
	WBMRemoveFailed,
	WBMDirectoryCreationFailed,
	WBMUnzipFailed,
}

export interface InstallStatus {
	DownloadBepInEx: boolean
	InstallBepInEx: boolean
	LaunchOption: boolean
	LaunchGame: boolean
	DownloadWbmZip: boolean
	InstallWbm: boolean
	Done: boolean
}
