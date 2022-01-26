<script lang="ts">
	// components also used outside
	import HomeButton from "../../components/HomeButton.svelte"
	import Spinner from "../../components/Spinner.svelte"

	// components only used here
	import Steps from "./Steps.svelte"
	import Interrupts from "./Interrupts.svelte"
	import Complete from "./Complete.svelte"

	// tauri stuff
	import { invoke } from "@tauri-apps/api/tauri"
	import { listen } from "@tauri-apps/api/event"
	import { open as dialogOpen } from "@tauri-apps/api/dialog"

	// types
	import { COMMANDS, EVENTS } from "../../constants"
	import { InstallResult, InstallSteps } from "./types"
	import type { InstallStatus } from "./types"

	//
	// variables
	//

	let lastReturnStatus: InstallResult = undefined
	let wasInstallButtonClicked = false
	let spinCog = false

	let installStatus: InstallStatus = {
		DownloadBepInEx: false,
		InstallBepInEx: false,
		LaunchOption: false,
		LaunchGame: false,
		DownloadWbmZip: false,
		InstallWbm: false,
		Done: false,
	}

	//
	// functions
	//

	function _install(gamePath = "") {
		wasInstallButtonClicked = true
		spinCog = true

		invoke<InstallResult>(COMMANDS.INSTALL, { gamePath }).then((res) => {
			lastReturnStatus = res

			switch (res) {
				case InstallResult.NoErr: {
					break
				}

				case InstallResult.FailedToGetGamePath: {
					break
				}

				case InstallResult.UnsupportedOS: {
					break
				}

				case InstallResult.BepInExDownloadFailed: {
					break
				}

				case InstallResult.BepInExUnzipFailed: {
					break
				}

				case InstallResult.SetLaunchOption: {
					break
				}

				case InstallResult.LaunchGame: {
					break
				}

				case InstallResult.WBMDownloadFailed: {
					break
				}

				case InstallResult.WBMRemoveFailed: {
					break
				}

				case InstallResult.WBMDirectoryCreationFailed: {
					break
				}

				case InstallResult.WBMUnzipFailed: {
					break
				}

				case InstallResult.NoErr: {
					break
				}
			}
		})
	}

	function install() {
		_install()
	}

	function selectGamePathAndInstall() {
		dialogOpen({ directory: true, multiple: false }).then((gamePath) => {
			_install(gamePath as string)
		})
	}

	//
	// Event listener
	//

	listen<InstallSteps>(EVENTS.INSTALL, (event) => {
		switch (event.payload) {
			case InstallSteps.DownloadBepInEx: {
				installStatus.DownloadBepInEx = true
				break
			}

			case InstallSteps.InstallBepInEx: {
				installStatus.InstallBepInEx = true
				break
			}

			case InstallSteps.LaunchOption: {
				installStatus.LaunchOption = true
				break
			}

			case InstallSteps.LaunchGame: {
				installStatus.LaunchGame = true
				break
			}

			case InstallSteps.DownloadWbmZip: {
				installStatus.DownloadWbmZip = true
				break
			}

			case InstallSteps.InstallWbm: {
				installStatus.InstallWbm = true
				break
			}

			case InstallSteps.Done: {
				spinCog = false
				installStatus.Done = true
				break
			}
		}
	})
</script>

<!-- Allow user to go back to home until they click the install button -->
{#if !wasInstallButtonClicked}
	<HomeButton />
{/if}

<div class="install-page">
	<Spinner activated={spinCog} />

	{#if !wasInstallButtonClicked}
		<button on:click={install}>Install!</button>
	{/if}

	<!-- show only when the install button is clicked -->
	{#if wasInstallButtonClicked}
		<Steps {installStatus} />
		<Interrupts {installStatus} {lastReturnStatus} {selectGamePathAndInstall} />

		{#if installStatus.Done}
			<Complete />
		{/if}
	{/if}
</div>

<style lang="scss">
	@import "./button.scss";

	.install-page {
		@apply flex flex-col items-center;
	}
</style>
