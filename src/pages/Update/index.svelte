<script lang="ts">
	import HomeButton from "../../components/HomeButton.svelte"

	import { invoke } from "@tauri-apps/api/tauri"
	import { COMMANDS } from "../../constants"

	function update() {
		invoke(COMMANDS.UPDATE)
	}
</script>

<HomeButton />
<br />
<button on:click={update}>Update!</button>
