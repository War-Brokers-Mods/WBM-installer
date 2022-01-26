<script lang="ts">
	import { copy } from "svelte-copy"
	import { open as shellOpen } from "@tauri-apps/api/shell"

	import type { InstallStatus } from "./types"
	import { InstallResult } from "./types"

	export let lastReturnStatus: InstallResult
	export let installStatus: InstallStatus
	export let selectGamePathAndInstall: () => void
</script>

<div class="interrupts">
	<!-- set game launch option -->
	{#if installStatus.LaunchOption}
		<span
			use:copy={"./run_bepinex.sh %command%"}
			on:svelte-copy={(event) => alert(event.detail)}
		>
			click to copy
		</span>
	{/if}

	<!-- if the game was not found in the default install location  -->
	{#if lastReturnStatus == InstallResult.FailedToGetGamePath}
		<p>
			Default game install location was not found :(
			<br />
			Press the button select the folder
			<!-- svelte-ignore a11y-invalid-attribute -->
			<a
				class="link"
				alt="how to find game install location"
				href="javascript:;"
				on:click={() => {
					shellOpen(
						"https://github.com/War-Brokers-Mods/WBM/blob/master/images/local_files.png"
					)
				}}
			>
				where the game is installed
			</a>.
		</p>

		<button on:click={selectGamePathAndInstall}>
			Select folder and Install
		</button>
	{/if}
</div>

<style lang="scss">
	.interrupts {
		@apply mt-2;

		@import "./button.scss";

		p {
			@apply text-center;
		}

		span {
			@apply select-all rounded pl-1 pr-1 border-2 border-neutral-600 font-bold;
		}
	}
</style>
