<script lang="ts">
	import HomeButton from "../../components/HomeButton.svelte"
	import Spinner from "../../components/Spinner.svelte"

	import { invoke } from "@tauri-apps/api/tauri"
	import { COMMANDS } from "../../constants"

	let activateSpinner = false

	function update() {
		activateSpinner = true
		invoke(COMMANDS.UPDATE)
	}
</script>

<HomeButton />

<div class="update-page">
	<Spinner activated={activateSpinner} />
	<button on:click={update}>Update!</button>
</div>

<style lang="scss">
	.update-page {
		@apply flex flex-col items-center;
	}
</style>
