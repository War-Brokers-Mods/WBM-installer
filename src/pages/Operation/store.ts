import { writable } from "svelte/store"
import type { InstallErr, RemoveErr } from "./types"

const wasButtonClicked = writable(false)
const spinCog = writable(false)

const gamePath = writable("")

const lastInstallErr = writable<InstallErr>(undefined)
const lastRemoveErr = writable<RemoveErr>(undefined)

const wasInstallSuccessful = writable(false)
const wasRemoveSuccessful = writable(false)

export default {
	wasButtonClicked,
	spinCog,

	gamePath,

	lastInstallErr,
	lastRemoveErr,

	wasInstallSuccessful,
	wasRemoveSuccessful,
}
