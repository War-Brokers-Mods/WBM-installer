<script lang="ts">
	import HomeButton from "../../components/HomeButton.svelte"
	import Spinner from "../../components/Spinner.svelte"

	import { invoke } from "@tauri-apps/api/tauri"
	import { listen } from "@tauri-apps/api/event"

	import { COMMANDS, EVENTS } from "../../constants"

	enum Steps {
		DownloadBepInEx,
		DownloadWbmZip,
		Done,
	}

	//
	//
	//

	let showButtons = false
	let spinCog = false
	let stepsStatus = {
		DownloadBepInEx: false,
		DownloadWbmZip: false,
		Done: false,
	}

	listen<Steps>(EVENTS.INSTALL, (event) => {
		switch (event.payload) {
			case Steps.DownloadBepInEx: {
				stepsStatus.DownloadBepInEx = true
				break
			}

			case Steps.DownloadWbmZip: {
				stepsStatus.DownloadWbmZip = true
				break
			}

			case Steps.Done: {
				spinCog = false
				stepsStatus.Done = true
				break
			}
		}
	})

	function install() {
		showButtons = true
		spinCog = true
		invoke(COMMANDS.INSTALL)
	}
</script>

{#if !showButtons}
	<HomeButton />
{/if}

<div class="install-page">
	<Spinner activated={spinCog} />

	{#if !showButtons}
		<button on:click={install}>Install!</button>
	{/if}

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
</div>

<style lang="scss">
	.install-page {
		@apply flex flex-col items-center;
	}

	button {
		/* text */
		@apply text-white font-sans font-semibold tracking-wide text-xl;

		/* style */
		@apply pt-2 pb-2 pl-3 pr-3 rounded bg-red-600;
	}
</style>
