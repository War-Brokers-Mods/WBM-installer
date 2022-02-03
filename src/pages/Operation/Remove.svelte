<script lang="ts">
	/**
	 * This component is shown during the removal of the War Brokers Mods.
	 * It asks for user input when needed and shows error when something's gone wrong.
	 */

	import store from "./store"
	import { RemoveErr } from "./types"
	import { selectGamePathAndRun } from "./logic"

	let lastRemoveErr: RemoveErr = undefined
	let wasRemoveSuccessful: boolean = false

	store.lastRemoveErr.subscribe((value) => {
		lastRemoveErr = value
	})

	store.wasRemoveSuccessful.subscribe((value) => {
		wasRemoveSuccessful = value
	})

	// todo: Reset steam launch option.
</script>

<div class="remove">
	{#if wasRemoveSuccessful}
		WBM Removed!
		<br />
		You may now close the installer.
	{:else if false}
		Reset steam launch options.

		<br />
		<br />

		<img alt="where to find property settings" src="/img/properties.png" />

		<br />

		<img alt="where to find launch option" src="/img/launch_option.png" />

		<br />

		<button
			on:click|once={() => {
				store.lastRemoveErr.set(undefined)
				store.wasRemoveSuccessful.set(true)
			}}
		>
			Continue!
		</button>
	{:else if lastRemoveErr == RemoveErr.FailedToGetGamePath}
		Where is the game folder?

		<br />
		<br />

		<button
			on:click|once={() => {
				selectGamePathAndRun("remove")
				lastRemoveErr = undefined
			}}
		>
			Select game path
		</button>
	{/if}
</div>

<style lang="scss">
	@import "./styles/button.scss";

	.remove {
		/* make install section scrollable */
		@apply w-full h-40 text-center overflow-y-auto overflow-x-hidden;

		img {
			/* center image */
			@apply block ml-auto mr-auto;
		}
	}
</style>
