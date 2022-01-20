<script lang="ts">
	import HomeButton from "../../components/HomeButton.svelte"
	import Spinner from "../../components/Spinner.svelte"

	import { invoke } from "@tauri-apps/api/tauri"
	import { COMMANDS } from "../../constants"

	let isRunning = false

	function update() {
		isRunning = true
		invoke(COMMANDS.UPDATE)
	}
</script>

{#if !isRunning}
	<HomeButton />
{/if}

<div class="update-page">
	<Spinner activated={isRunning} />
	<button on:click={update}>Update!</button>
</div>

<style lang="scss">
	.update-page {
		@apply flex flex-col items-center;
	}
</style>
