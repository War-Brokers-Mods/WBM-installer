<script lang="ts">
	/**
	 * This component is shown during installation.
	 * It asks for user input when needed and shows error when something's gone wrong.
	 */

	import store from "./store"
	import { InstallErr } from "./types"
	import { selectGamePathAndRun } from "./logic"

	import { listen } from "@tauri-apps/api/event"
	import { open as shellOpen } from "@tauri-apps/api/shell"
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
		<br />
		<br />
		<!-- svelte-ignore a11y-invalid-attribute -->
		<a
			href="javascript:;"
			on:click={() => {
				shellOpen("https://github.com/War-Brokers-Mods/WBM#usage")
			}}
		>
			Learn how to use the mod
		</a>
	{:else if lastInstallErr == InstallErr.UnsupportedOS}
		Operating System not supported.
		<br />
		WBM Installer is only available in Windows, Mac, and Linux.
	{:else if lastInstallErr == InstallErr.FailedToGetGamePath}
		Where is the game folder?

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
		Paste this to the steam launch options <b>(click to copy)</b>:<br />
		<code
			style="cursor: pointer"
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
			Continue!
		</button>
	{:else if lastInstallErr == InstallErr.FailedToSendLaunchOption}
		Failed to receive steam launch option data :(
	{/if}
</div>

<style lang="scss">
	@import "./styles/anchor.scss";
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
