<script lang="ts">
	import HomeButton from "../../components/HomeButton.svelte"

	import { invoke } from "@tauri-apps/api/tauri"
	import { COMMANDS } from "../../constants"

	function install() {
		invoke(COMMANDS.INSTALL)
	}
</script>

<HomeButton />
<br />
<button on:click={install}>Install!</button>
