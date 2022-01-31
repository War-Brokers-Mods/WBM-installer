/**
 * Must be synced with `src-tauri/src/commands/install/types.rs`
 */
export enum InstallErr {
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
