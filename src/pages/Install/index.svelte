<script lang="ts">
	import HomeButton from "../../components/HomeButton.svelte"
	import Spinner from "../../components/Spinner.svelte"
	import { copy } from "svelte-copy"

	import { invoke } from "@tauri-apps/api/tauri"
	import { listen } from "@tauri-apps/api/event"
	import { open as dialogOpen } from "@tauri-apps/api/dialog"
	import { open as shellOpen } from "@tauri-apps/api/shell"

	import { COMMANDS, EVENTS } from "../../constants"

	// types of event
	enum InstallSteps {
		DownloadBepInEx,
		InstallBepInEx,
		LaunchOption,
		LaunchGame,
		DownloadWbmZip,
		InstallWbm,
		Done,
	}

	// types of install command return codes
	enum InstallResult {
		NoErr,
		FailedToGetGamePath,
		UnsupportedOS,
		BepInExDownloadFailed,
		BepInExUnzipFailed,
		WBMDownloadFailed,
		WBMRemoveFailed,
		WBMDirectoryCreationFailed,
		WBMUnzipFailed,
	}

	//
	// variables
	//

	let isInstallButtonClicked = false
	let wasDefaultGamePathFound = true
	let spinCog = false

	let stepsStatus = {
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

	//
	// Event listener
	//

	listen<InstallSteps>(EVENTS.INSTALL, (event) => {
		switch (event.payload) {
			case InstallSteps.DownloadBepInEx: {
				stepsStatus.DownloadBepInEx = true
				break
			}

			case InstallSteps.InstallBepInEx: {
				break
			}

			case InstallSteps.LaunchOption: {
				break
			}

			case InstallSteps.LaunchGame: {
				break
			}

			case InstallSteps.DownloadWbmZip: {
				stepsStatus.DownloadWbmZip = true
				break
			}

			case InstallSteps.InstallWbm: {
				break
			}

			case InstallSteps.Done: {
				spinCog = false
				stepsStatus.Done = true
				break
			}
		}
	})
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

	<!-- if the game was not found in the default install location  -->
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

	{#if isInstallButtonClicked}
		<div class="steps">
			<div class="step {stepsStatus.DownloadBepInEx}">
				<div class="number">1</div>
				<div class="label">Install BepInEx</div>
			</div>

			<div class="step {stepsStatus.LaunchGame && 'done'}">
				<div class="number">2</div>
				<div class="label">Launch game</div>
			</div>

			<div class="step {stepsStatus.InstallWbm && 'done'}">
				<div class="number">3</div>
				<div class="label">Install Mod</div>
			</div>
		</div>
	{/if}

	<div class="instructions">
		{#if true}
			<span
				use:copy={"./run_bepinex.sh %command%"}
				on:svelte-copy={(event) => alert(event.detail)}
			>
				click to copy
			</span>
		{/if}
	</div>

	{#if stepsStatus.Done}
		<br />
		<p>
			You can also optionally setup
			<!-- svelte-ignore a11y-invalid-attribute -->
			<a
				class="link"
				href="javascript:;"
				on:click={() => {
					shellOpen(
						"https://github.com/War-Brokers-Mods/WBM#3-set-up-obs-optional"
					)
				}}
			>
				OBS overlays
			</a>
			for WB statistics.
		</p>
	{/if}
</div>

<style lang="scss">
	.install-page {
		@apply flex flex-col items-center;
	}

	.steps {
		@apply grid grid-cols-3 w-full gap-2 p-2;

		.step {
			/* layout */
			@apply flex flex-col text-center gap-2 p-2;
			/* style */
			@apply rounded-xl bg-neutral-600;

			.number {
				@apply text-4xl font-bold;
			}

			.label {
				@apply text-sm;
			}
		}

		.done {
			@apply bg-red-600;
		}
	}

	.instructions {
		@apply mt-2;

		span {
			@apply select-all rounded pl-1 pr-1 border-2 border-neutral-600 font-bold;
		}
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
