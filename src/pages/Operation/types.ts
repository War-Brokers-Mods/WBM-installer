/**
 * Must be synced with `src-tauri/src/commands/install/types.rs`
 */
export enum InstallErr {
	UnsupportedOS,
	FailedToGetGamePath,
	RemoveOldFilesFailed,
	BepInExDownloadFailed,
	BepInExUnzipFailed,
	WBMDownloadFailed,
	WBMDirectoryCreationFailed,
	WBMUnzipFailed,
	LaunchOptionNotSet,
	FailedToSendLaunchOption,
}

/**
 * Must be synced with `src-tauri/src/commands/remove/types.rs`
 */
export enum RemoveErr {
	FailedToGetGamePath,
	GamePathNotValid,
}

export enum OperationType {
	Install,
	Remove,
}
