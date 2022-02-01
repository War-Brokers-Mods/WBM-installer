<script lang="ts">
	import store from "./store"
	import { InstallErr } from "./types"
	import { install, selectGamePathAndRun } from "./logic"

	import { listen } from "@tauri-apps/api/event"

	import { getNotificationsContext } from "svelte-notifications"
	import { copy } from "svelte-copy"

	const { addNotification } = getNotificationsContext()

	let lastInstallErr: InstallErr = undefined
	let wasInstallSuccessful: boolean = false
	let launhOptionString: string = "loading..."

	store.lastInstallErr.subscribe((value) => {
		lastInstallErr = value
	})

	store.wasInstallSuccessful.subscribe((value) => {
		wasInstallSuccessful = value
	})

	listen<string>("launch-option-string", ({ payload }) => {
		launhOptionString = payload
	})
</script>

<div class="install">
	{#if wasInstallSuccessful}
		Install Success!
		<br />
		You may now close the installer.
	{:else if lastInstallErr == InstallErr.UnsupportedOS}
		Operating System not supported.
		<br />
		WBM Installer is only available in Windows, Mac, and Linux.
	{:else if lastInstallErr == InstallErr.FailedToGetGamePath}
		Failed to find game folder.

		<br />
		<br />

		<button
			on:click|once={() => {
				selectGamePathAndRun(install)
			}}
		>
			Select game path
		</button>
	{:else if lastInstallErr == InstallErr.RemoveOldFilesFailed}
		Failed to remove old files :(
	{:else if lastInstallErr == InstallErr.BepInExDownloadFailed}
		Failed to download BepInEx :(
	{:else if lastInstallErr == InstallErr.BepInExUnzipFailed}
		Failed to unzip BepInEx :(
	{:else if lastInstallErr == InstallErr.WBMDownloadFailed}
		Failed to download WBM :(
	{:else if lastInstallErr == InstallErr.WBMDirectoryCreationFailed}
		Failed to create WMB folder :(
	{:else if lastInstallErr == InstallErr.WBMUnzipFailed}
		Failed to unzip WBM :(
	{:else if lastInstallErr == InstallErr.LaunchOptionNotSet}
		<!-- todo: implement click to copy -->
		Copy and paste the following text to steam launch option: "<code>
			{launhOptionString}
		</code>"

		<button
			use:copy={"Hello World"}
			on:click={() =>
				addNotification({
					text: "Copy",
					position: "bottom-center",
				})}
		>
			Copy
		</button>

		<img alt="where to find property settings" src="/img/properties.png" />

		<button
			on:click={() => {
				install()
			}}
		>
			Done! Continue!
		</button>
	{:else if lastInstallErr == InstallErr.FailedToSendLaunchOption}
		Failed to receive steam launch option data :(
	{/if}
</div>
<!-- Handle lastInstallErr change -->

<!-- On install complete -->
<style lang="scss">
	@import "./styles/button.scss";

	.install {
		@apply text-center;
	}
</style>
