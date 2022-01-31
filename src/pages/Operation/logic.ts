import { invoke } from "@tauri-apps/api/tauri"
import { open as dialogOpen } from "@tauri-apps/api/dialog"

import { COMMANDS } from "../../constants"
import store from "./store"

import type { InstallErr, RemoveErr } from "./types"

function buttonClicked() {
	store.wasButtonClicked.set(true)
	store.spinCog.set(true)
}

/**
 * Calls the install command in the backend.
 *
 * @param {string} gamePath - Absolute path to the game directory in the steam library. Leave it empty to use default location.
 */
export function install(gamePath: string = "") {
	buttonClicked()

	if (!gamePath) {
		store.gamePath.update((value) => {
			gamePath = value
			return value
		})
	}

	invoke(COMMANDS.INSTALL, { gamePath })
		.then(() => {
			store.wasInstallSuccessful.set(true)
		})
		.catch((err: InstallErr) => {
			console.error(err)

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
			console.error(err)

			store.lastRemoveErr.set(err)
		})
}

/**
 * Opens a file selection dialog for the user to manually select the game directory.
 * Called when the default game location was not found.
 *
 * @param {(gamePath?: string) => void} f - Function that will run after selecting a directory. Expected to be either the {@link install} function or the {@link remove} function.
 */
export function selectGamePathAndRun(f: (gamePath?: string) => void) {
	dialogOpen({ directory: true, multiple: false }).then((value: string) => {
		store.gamePath.set(value)

		f(value)
	})
}
