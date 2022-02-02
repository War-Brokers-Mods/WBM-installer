<script lang="ts">
	import store from "./store"
	import { InstallErr } from "./types"
	import { selectGamePathAndRun } from "./logic"

	import { listen } from "@tauri-apps/api/event"
	import { writeText } from "@tauri-apps/api/clipboard"

	import { getNotificationsContext } from "svelte-notifications"

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
				selectGamePathAndRun("install")
				lastInstallErr = undefined
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
		Paste the following text to steam launch option (click to copy):<br />
		<code
			on:click|once={() => {
				writeText(launhOptionString)

				addNotification({
					text: "Copied",
					position: "bottom-center",
					type: "success",
					removeAfter: 5000,
				})
			}}>{launhOptionString}</code
		>

		<br />
		<br />

		<img alt="where to find property settings" src="/img/properties.png" />

		<br />

		<img alt="where to find launch option" src="/img/launch_option.png" />

		<br />

		<button
			on:click|once={() => {
				store.lastInstallErr.set(undefined)
				store.wasInstallSuccessful.set(true)
			}}
		>
			Done!
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
		/* make install section scrollable */
		@apply w-full h-40 text-center overflow-y-auto overflow-x-hidden;

		code {
			color: rgb(238, 238, 238);
			background-color: rgb(68, 68, 68);
			border: 1px solid rgb(102, 102, 102);
			border-radius: 3px;
			padding: 1px 3px;
		}

		img {
			/* center image */
			@apply block ml-auto mr-auto;
		}
	}
</style>
