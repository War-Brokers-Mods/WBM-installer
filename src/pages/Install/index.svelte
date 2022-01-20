<script lang="ts">
	import HomeButton from "../../components/HomeButton.svelte"
	import Spinner from "../../components/Spinner.svelte"

	import { invoke } from "@tauri-apps/api/tauri"
	import { COMMANDS } from "../../constants"

	let activateSpinner = false

	function install() {
		activateSpinner = true
		invoke(COMMANDS.INSTALL)
	}
</script>

<HomeButton />

<div class="install-page">
	<Spinner activated={activateSpinner} />

	<button on:click={install}>Install!</button>
</div>

<style lang="scss">
	.install-page {
		@apply flex flex-col items-center;
	}
</style>
