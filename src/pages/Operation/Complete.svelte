<script lang="ts">
	import { open as shellOpen } from "@tauri-apps/api/shell"

	import { OperationType } from "./types"

	//
	// Variables
	//

	export let operationType: OperationType
</script>

{#if operationType == OperationType.Install}
	<p>
		You can also optionally setup
		<!-- svelte-ignore a11y-invalid-attribute -->
		<a
			href="javascript:;"
			on:click={() => {
				shellOpen(
					"https://github.com/War-Brokers-Mods/WBM#3-set-up-obs-optional"
				)
			}}
		>
			OBS overlays
		</a>
		for WB statistics.
	</p>
{:else}
	<p>Remove launch option if you're using macOS or linux.</p>
{/if}

<style lang="scss">
	p {
		@apply text-center;
	}

	a {
		@apply text-blue-400;

		&:hover {
			@apply text-blue-500;
		}
	}
</style>
