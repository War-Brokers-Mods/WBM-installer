<script lang="ts">
	import type { InstallStatus } from "./types"

	export let installStatus: InstallStatus
</script>

<div class="steps">
	<div class="step {installStatus.DownloadBepInEx}">
		<div class="number">1</div>
		<div class="label">Install BepInEx</div>
	</div>

	<div class="step {installStatus.LaunchGame && 'done'}">
		<div class="number">2</div>
		<div class="label">Launch game</div>
	</div>

	<div class="step {installStatus.InstallWbm && 'done'}">
		<div class="number">3</div>
		<div class="label">Install Mod</div>
	</div>
</div>

<style lang="scss">
	.steps {
		@apply grid grid-cols-3 w-full gap-2 p-2;

		.step {
			/* layout */
			@apply flex flex-col text-center gap-2 p-2;
			/* style */
			@apply rounded-xl bg-neutral-600;

			.number {
				@apply text-4xl font-bold;
			}

			.label {
				@apply text-sm;
			}
		}

		.done {
			@apply bg-red-600;
		}
	}
</style>
