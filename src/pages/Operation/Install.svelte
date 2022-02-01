<script lang="ts">
	import store from "./store"
	import { InstallErr } from "./types"
	import { install, selectGamePathAndRun } from "./logic"

	let lastInstallErr: InstallErr = undefined
	let wasInstallSuccessful: boolean = false

	store.lastInstallErr.subscribe((value) => {
		lastInstallErr = value
	})

	store.wasInstallSuccessful.subscribe((value) => {
		wasInstallSuccessful = value
	})
</script>

<div class="install">
	{#if lastInstallErr == InstallErr.UnsupportedOS}
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
		<!-- todo: implement -->
		Copy and paste the following text to ...

		<img alt="where to find property settings" src="/img/properties.png" />

		<button
			on:click={() => {
				install()
			}}
		>
			Done! Continue!
		</button>
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
