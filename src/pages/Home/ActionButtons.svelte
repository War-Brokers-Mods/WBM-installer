<script lang="ts">
	import FaTasks from "svelte-icons/fa/FaTasks.svelte"
	import FaArrowAltCircleRight from "svelte-icons/fa/FaArrowAltCircleRight.svelte"
	import Update from "svelte-icons/fa/FaRedoAlt.svelte"
	import { ROUTES } from "../../constants"
</script>

<div class="main-buttons-container">
	<a href={ROUTES.STATUS}>
		<div class="icon"><FaTasks /></div>
		<div>Status</div>
	</a>

	<a href={ROUTES.INSTALL}>
		<div class="icon"><FaArrowAltCircleRight /></div>
		<div>Install</div>
	</a>

	<a href={ROUTES.UPDATE}>
		<div class="icon"><Update /></div>
		<div>Update</div>
	</a>
</div>

<style lang="scss">
	.main-buttons-container {
		@apply grid gap-6 grid-cols-3;

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
