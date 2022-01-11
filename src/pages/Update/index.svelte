<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri"
	import { ROUTES, COMMANDS } from "../../constants"

	function update() {
		invoke(COMMANDS.UPDATE)
	}
</script>

<a href={ROUTES.HOME}>Home</a>
<br />
<button on:click={update}>Update!</button>
