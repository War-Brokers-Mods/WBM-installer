<script lang="ts">
	import FaTasks from "svelte-icons/fa/FaTasks.svelte"
	import FaArrowAltCircleRight from "svelte-icons/fa/FaArrowAltCircleRight.svelte"
	import Update from "svelte-icons/fa/FaRedoAlt.svelte"
</script>

<div class="main-buttons-container">
	<button>
		<div class="icon"><FaTasks /></div>
		<div>Status</div>
	</button>
	<button>
		<div class="icon"><FaArrowAltCircleRight /></div>
		<div>Install</div>
	</button>
	<button>
		<div class="icon"><Update /></div>
		<div>Update</div>
	</button>
</div>

<style lang="scss">
	.main-buttons-container {
		@apply grid gap-6 grid-cols-3;

		button {
			@apply p-4 w-24 h-24 rounded-xl bg-red-500 text-white font-normal;

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
