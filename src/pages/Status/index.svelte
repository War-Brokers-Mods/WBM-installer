<script lang="ts">
	// [Sync]: must be synced with `src-tauri/src/commands/status.rs`
	import HomeButton from "../../components/HomeButton.svelte"
	import Spinner from "../../components/Spinner.svelte"

	import { invoke } from "@tauri-apps/api/tauri"
	import { open as openShell } from "@tauri-apps/api/shell"
	import { COMMANDS } from "../../constants"

	// [Sync]
	enum STATUS_REQUEST {
		LATEST_VERSION = "LATEST_VERSION",
		GAME_PATH = "GAME_PATH",
	}

	// [Sync]
	interface StatusDataRaw {
		latest_release_version?: string
		game_path?: string
	}

	// [Sync]
	interface StatusData {
		latestReleaseVersion?: LatestReleaseVersion
		gamePath?: string
	}

	interface LatestReleaseVersion {
		name: string
		url: string
	}

	//
	//
	//

	let statusData: StatusData = {}
	let isRunning = false

	function _requestStatus(
		reqType: STATUS_REQUEST,
		f: (res: StatusDataRaw) => void,
		f2?: (res: any) => void
	) {
		// fallback to default error handling if f2 is not defined
		f2 ||= (err) => {
			console.error(err)
		}

		invoke(COMMANDS.STATUS, { reqType }).then(f).catch(f2)
	}

	async function status() {
		isRunning = true
		_requestStatus(STATUS_REQUEST.LATEST_VERSION, (res) => {
			const data = (JSON.parse(res.latest_release_version) as any[])[0]
			statusData.latestReleaseVersion = {
				name: data.name,
				url: data.html_url,
			}
		})

		_requestStatus(STATUS_REQUEST.GAME_PATH, (res) => {
			statusData.gamePath = res.game_path || "steam apps folder not found"
		})
	}
</script>

{#if !isRunning}
	<HomeButton />
{/if}

<div class="status-page">
	<Spinner activated={isRunning} />

	<div>
		<button on:click={status}>Check status!</button>

		<br />
		<br />

		Latest version:
		{#if statusData.latestReleaseVersion}
			<div
				on:click={() => {
					console.log(statusData.latestReleaseVersion.url)
					openShell(statusData.latestReleaseVersion.url)
				}}
				style="display: inline-block; cursor: pointer"
			>
				{statusData.latestReleaseVersion.name}
			</div>
		{/if}

		<br />

		game Path:
		{#if statusData.gamePath}
			{statusData.gamePath}
		{/if}
	</div>
</div>

<style lang="scss">
	.status-page {
		@apply flex flex-col items-center;
	}
</style>
