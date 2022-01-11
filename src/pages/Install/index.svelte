<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri"
	import { ROUTES, COMMANDS } from "../../constants"

	function install() {
		invoke(COMMANDS.INSTALL)
	}
</script>

<a href={ROUTES.HOME}>Home</a>
<br />
<button on:click={install}>Install!</button>
