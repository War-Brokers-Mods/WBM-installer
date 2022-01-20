<script lang="ts">
	import FaHome from "svelte-icons/fa/FaHome.svelte"
	import { ROUTES } from "../constants"
</script>

<a href={ROUTES.HOME}>
	<FaHome />
</a>

<style lang="scss">
	a {
		@apply absolute ml-2 p-2 w-10 h-10 bg-red-500 text-white;

		&:hover {
			@apply bg-red-700;
		}

		&:active {
			@apply bg-red-800;
		}
	}
</style>
