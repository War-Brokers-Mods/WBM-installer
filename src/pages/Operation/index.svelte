<script lang="ts">
	// components also used outside
	import HomeButton from "../../components/HomeButton.svelte"
	import Spinner from "../../components/Spinner.svelte"

	// components only used here
	import Interrupts from "./Interrupts.svelte"
	import Complete from "./Complete.svelte"

	// tauri stuff
	import { invoke } from "@tauri-apps/api/tauri"
	import { open as dialogOpen } from "@tauri-apps/api/dialog"

	// svelte stuff
	import { querystring } from "svelte-spa-router"

	// types
	import { COMMANDS } from "../../constants"
	import { InstallErr, RemoveErr } from "./types"

	interface InstallArgs {
		gamePath: string
	}

	enum OperationType {
		Install,
		Remove,
	}

	//
	// variables
	//

	const operationType: OperationType = $querystring.includes("install")
		? OperationType.Install
		: OperationType.Remove
	let _gamePath = "" // not used directly

	let lastInstallErrStaus: InstallErr = undefined
	let lastRemoveErrStatus: RemoveErr = undefined

	let wasButtonClicked = false // if the install/remove button was clicked or not
	let spinCog = false

	let installSuccess = false
	let removeSuccess = false

	//
	// functions
	//

	/**
	 * only used inside other install functions.
	 * Is never called directly.
	 *
	 * @param {InstallArgs} args
	 */
	function _install(args: InstallArgs) {
		wasButtonClicked = true
		spinCog = true

		invoke<InstallErr>(COMMANDS.INSTALL, args as any)
			.then((res) => {
				switch (res) {
					case InstallErr.FailedToGetGamePath: {
						break
					}
				}
			})
			.catch((err: InstallErr) => {
				console.log(typeof err, err)
			})
	}

	/**
	 * entry point
	 */
	function install() {
		_install({
			gamePath: _gamePath,
		})
	}

	/**
	 * called when default game path was not found.
	 */
	function selectGamePathAndInstall() {
		dialogOpen({ directory: true, multiple: false }).then((value) => {
			_gamePath = value as string
			_install({
				gamePath: _gamePath,
			})
		})
	}

	/**
	 * called after setting the steam launch option.
	 */
	function setSteamLaunchOptionAndInstall() {
		_install({
			gamePath: _gamePath,
		})
	}
</script>

<!-- Allow user to go back to home until they click the install button -->
{#if !wasButtonClicked}
	<HomeButton />
{/if}

{#if operationType == OperationType.Install}
	<div class="install-page">
		<Spinner activated={spinCog} />

		{#if !wasButtonClicked}
			<button on:click={install}>Install!</button>
		{/if}

		<!-- show only when the install button is clicked -->
		{#if wasButtonClicked}
			<Interrupts
				lastErrStaus={lastInstallErrStaus}
				{selectGamePathAndInstall}
				{setSteamLaunchOptionAndInstall}
			/>

			{#if installSuccess}
				<Complete />
			{/if}
		{/if}
	</div>
{:else}
	<div class="remove-page">
		<Spinner activated={spinCog} />

		{#if !wasButtonClicked}
			<button on:click={install}>Remove!</button>
		{/if}

		<!-- show only when the install button is clicked -->
		{#if wasButtonClicked}
			<Interrupts
				lastErrStaus={lastInstallErrStaus}
				{selectGamePathAndInstall}
				{setSteamLaunchOptionAndInstall}
			/>

			{#if installSuccess}
				<Complete />
			{/if}
		{/if}
	</div>
{/if}

<style lang="scss">
	@import "./styles/button.scss";

	.install-page {
		@apply flex flex-col items-center;
	}

	.remove-page {
		@apply flex flex-col items-center;
	}
</style>
