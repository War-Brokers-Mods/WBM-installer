<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri"
	import { open } from "@tauri-apps/api/shell"
	import { ROUTES, COMMANDS, STATUS_REQUEST } from "../../constants"

	interface StatusData {
		latestReleaseVersion?: LatestReleaseVersion
		gamePath?: string
	}

	interface LatestReleaseVersion {
		name: string
		url: string
	}

	let statusData: StatusData = {}

	async function status() {
		invoke(COMMANDS.STATUS, {
			reqType: STATUS_REQUEST.LATEST_VERSION,
		})
			.then((res: any) => {
				const data = (JSON.parse(res.latest_release_version) as any[])[0]
				statusData.latestReleaseVersion = {
					name: data.name,
					url: data.html_url,
				}
			})
			.catch((err) => {
				console.error(err)
			})
	}
</script>

<a href={ROUTES.HOME}>Home</a>
<br />
<button on:click={status}>Check status!</button>
<br />

Latest version:
{#if statusData.latestReleaseVersion}
	<div
		on:click={() => {
			console.log(statusData.latestReleaseVersion.url)
			open(statusData.latestReleaseVersion.url)
		}}
		style="display: inline-block; cursor: pointer"
	>
		{statusData.latestReleaseVersion.name}
	</div>
{/if}
