<script lang="ts">
	import FaDownload from "svelte-icons/fa/FaDownload.svelte"
	import MdDeleteSweep from "svelte-icons/md/MdDeleteSweep.svelte"

	import { ROUTES } from "../../constants"
</script>

<div class="main-buttons-container">
	<a href={ROUTES.INSTALL}>
		<div class="icon">
			<FaDownload />
		</div>
		<div>Install</div>
	</a>

	<a href={ROUTES.REMOVE}>
		<div class="icon">
			<MdDeleteSweep />
		</div>
		<div>Remove</div>
	</a>
</div>

<style lang="scss">
	.main-buttons-container {
		@apply w-full flex justify-evenly;

		a {
			@apply p-4 w-24 h-24 text-center rounded-xl bg-red-500 text-white font-normal;

			.icon {
				@apply w-full h-6 mb-2;
			}

			&:hover {
				@apply bg-red-700;
			}

			&:active {
				@apply bg-red-800;
			}
		}
	}
</style>
