import { invoke } from "@tauri-apps/api/tauri"
import { open as dialogOpen } from "@tauri-apps/api/dialog"

import { COMMANDS } from "../../constants"
import store from "./store"

import type { InstallErr, RemoveErr } from "./types"

function buttonClicked() {
	store.wasButtonClicked.set(true)
	store.spinCog.set(true)
}

interface InstallArgs {
	gamePath?: string
	isLaunchOptionSet?: boolean
}

/**
 * Calls the install command in the backend.
 *
 * @param {InstallArgs} gamePath - Absolute path to the game directory in the steam library. Leave it empty to use default location.
 */
export function install(args: InstallArgs = {}) {
	buttonClicked()

	// get stored gamePath if it's empty
	if (!args.gamePath) {
		store.gamePath.update((value) => {
			args.gamePath = value
			return value
		})
	}

	invoke(COMMANDS.INSTALL, {
		gamePath: args.gamePath || "",
		isLaunchOptionSet: args.isLaunchOptionSet || false,
	})
		.then(() => {
			store.wasInstallSuccessful.set(true)
		})
		.catch((err: InstallErr) => {
			store.lastInstallErr.set(err)
		})
}

/**
 * Calls the remove command in the backend.
 *
 * @param {string} gamePath - Absolute path to the game directory in the steam library. Leave it empty to use default location.
 */
export function remove(gamePath: string = "") {
	buttonClicked()

	if (gamePath) {
		store.gamePath.update((value) => {
			gamePath = value
			return value
		})
	}

	invoke(COMMANDS.REMOVE, { gamePath })
		.then(() => {
			store.wasRemoveSuccessful.set(true)
		})
		.catch((err: RemoveErr) => {
			store.lastRemoveErr.set(err)
		})
}

/**
 * Opens a file selection dialog for the user to manually select the game directory.
 * Called when the default game location was not found.
 *
 * @param {"install" | "remove"} type - Function that will run after selecting a directory. Expected to be either the {@link install} function or the {@link remove} function.
 */
export function selectGamePathAndRun(type: "install" | "remove") {
	dialogOpen({ directory: true, multiple: false }).then((value: string) => {
		store.gamePath.set(value)

		switch (type) {
			case "install": {
				install({ gamePath: value })
			}

			case "remove": {
				remove(value)
			}
		}
	})
}
