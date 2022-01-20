<script lang="ts">
	import HomeButton from "../../components/HomeButton.svelte"
	import Spinner from "../../components/Spinner.svelte"

	import { invoke } from "@tauri-apps/api/tauri"
	import { listen } from "@tauri-apps/api/event"

	import { COMMANDS, EVENTS } from "../../constants"

	enum InstallSteps {
		DownloadBepInEx = 0,
		DownloadWbmZip,
	}

	interface InstallEventPayload {
		complete: InstallSteps
	}

	//
	//
	//

	let isRunning = false

	listen<InstallEventPayload>(EVENTS.INSTALL, (event) => {
		console.log(event.payload.complete)
	})

	function install() {
		isRunning = true
		invoke(COMMANDS.INSTALL)
	}
</script>

{#if !isRunning}
	<HomeButton />
{/if}

<div class="install-page">
	<Spinner activated={isRunning} />

	<button on:click={install}>Install!</button>
</div>

<style lang="scss">
	.install-page {
		@apply flex flex-col items-center;
	}
</style>
