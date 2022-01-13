export enum ROUTES {
	HOME = "#/",
	STATUS = "#/status",
	INSTALL = "#/install",
	UPDATE = "#/update",
}

// https://tauri.studio/en/docs/usage/guides/command
export enum COMMANDS {
	STATUS = "status",
	INSTALL = "install",
	UPDATE = "update",
}

// must be synced with `src-tauri/src/commands/status.rs`
export enum STATUS_REQUEST {
	LATEST_VERSION = "LATEST_VERSION",
	GAME_PATH = "GAME_PATH",
}
