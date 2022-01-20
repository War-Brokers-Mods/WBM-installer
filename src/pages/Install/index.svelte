<script lang="ts">
	import HomeButton from "../../components/HomeButton.svelte"
	import Spinner from "../../components/Spinner.svelte"

	import { invoke } from "@tauri-apps/api/tauri"
	import { COMMANDS } from "../../constants"

	let isRunning = false

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
