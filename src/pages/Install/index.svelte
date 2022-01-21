<script lang="ts">
	import HomeButton from "../../components/HomeButton.svelte"
	import Spinner from "../../components/Spinner.svelte"

	import { invoke } from "@tauri-apps/api/tauri"
	import { listen } from "@tauri-apps/api/event"
	import { open as dialogOpen } from "@tauri-apps/api/dialog"
	import { open as shellOpen } from "@tauri-apps/api/shell"

	import { COMMANDS, EVENTS } from "../../constants"

	enum InstallSteps {
		DownloadBepInEx,
		InstallBepInEx,
		UnixLaunchOption,
		LaunchGame,
		DownloadWbmZip,
		InstallWbm,
		Done,
	}

	enum InstallResult {
		NoErr,
		FailedToGetGamePath,
	}

	//
	// variables
	//

	let isInstallButtonClicked = false
	let wasDefaultGamePathFound = true
	let spinCog = false

	let stepsStatus = {
		DownloadBepInEx: false,
		DownloadWbmZip: false,
		Done: false,
	}

	//
	// functions
	//

	listen<InstallSteps>(EVENTS.INSTALL, (event) => {
		switch (event.payload) {
			case InstallSteps.DownloadBepInEx: {
				stepsStatus.DownloadBepInEx = true
				break
			}

			case InstallSteps.DownloadWbmZip: {
				stepsStatus.DownloadWbmZip = true
				break
			}

			case InstallSteps.Done: {
				spinCog = false
				stepsStatus.Done = true
				break
			}
		}
	})

	function _install(gamePath = "") {
		isInstallButtonClicked = true
		spinCog = true

		invoke<InstallResult>(COMMANDS.INSTALL, { gamePath }).then((res) => {
			switch (res) {
				case InstallResult.NoErr: {
					break
				}

				case InstallResult.FailedToGetGamePath: {
					wasDefaultGamePathFound = false
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
</script>

<!-- Allow user to go back to home until they click the install button -->
{#if !isInstallButtonClicked}
	<HomeButton />
{/if}

<div class="install-page">
	<Spinner activated={spinCog} />

	<!-- hide install button when it's clicked -->
	{#if !isInstallButtonClicked}
		<button on:click={install}>Install!</button>
	{/if}

	{#if isInstallButtonClicked && !wasDefaultGamePathFound}
		<p>
			Default game install location was not found :(
			<br />
			Press the button select the folder
			<!-- svelte-ignore a11y-invalid-attribute -->
			<a
				class="link"
				alt="how to find game install location"
				href="javascript:;"
				on:click={() => {
					shellOpen(
						"https://github.com/War-Brokers-Mods/WBM/blob/master/images/local_files.png"
					)
				}}>where the game is installed</a
			>.
		</p>

		<button on:click={selectGamePathAndInstall}
			>Select folder and Install</button
		>
	{/if}

	<p>
		{#if stepsStatus.DownloadBepInEx}
			Download BepInEx!
		{/if}
		<br />
		{#if stepsStatus.DownloadWbmZip}
			Download WBM!
		{/if}
		<br />
		{#if stepsStatus.Done}
			Done!
		{/if}
	</p>
</div>

<style lang="scss">
	.install-page {
		@apply flex flex-col items-center;
	}

	p {
		@apply text-center;
	}

	a.link {
		@apply text-blue-400;

		&:hover {
			@apply text-blue-500;
		}
	}

	button {
		/* text */
		@apply text-white font-sans font-semibold tracking-wide text-xl;

		/* style */
		@apply pt-2 pb-2 pl-3 pr-3 rounded bg-red-600;
	}
</style>
