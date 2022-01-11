<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri"
	import { ROUTES, COMMANDS } from "../../constants"

	function status() {
		invoke(COMMANDS.STATUS)
	}
</script>

<a href={ROUTES.HOME}>Home</a>
<br />
<button on:click={status}>Check status!</button>
