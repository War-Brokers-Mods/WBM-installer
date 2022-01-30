<script lang="ts">
	import { open as shellOpen } from "@tauri-apps/api/shell"
</script>

<p>
	You can also optionally setup
	<!-- svelte-ignore a11y-invalid-attribute -->
	<a
		href="javascript:;"
		on:click={() => {
			shellOpen("https://github.com/War-Brokers-Mods/WBM#3-set-up-obs-optional")
		}}
	>
		OBS overlays
	</a>
	for WB statistics.
</p>

<style lang="scss">
	p {
		@apply text-center;

		a {
			@apply text-blue-400;

			&:hover {
				@apply text-blue-500;
			}
		}
	}
</style>
