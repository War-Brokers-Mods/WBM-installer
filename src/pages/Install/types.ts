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
	SetLaunchOption,
	LaunchGame,
	Skip, // only used for subcommands
}

export enum InstallErr {
	FailedToGetGamePath,
	UnsupportedOS,
	BepInExDownloadFailed,
	BepInExUnzipFailed,
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
