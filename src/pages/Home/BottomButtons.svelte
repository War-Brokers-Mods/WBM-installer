<script lang="ts">
	import FaGithub from "svelte-icons/fa/FaGithub.svelte"
	import FaDiscord from "svelte-icons/fa/FaDiscord.svelte"
	import FaBook from "svelte-icons/fa/FaBook.svelte"
</script>

<div class="small-button-container">
	<button><FaBook /></button>
	<button><FaGithub /></button>
	<button><FaDiscord /></button>
</div>

<style lang="scss">
	.small-button-container {
		@apply grid gap-3 grid-cols-3 mt-5;

		button {
			@apply w-6 h-6;

			&:hover {
				@apply text-gray-200;
			}

			&:active {
				@apply text-gray-300;
			}
		}
	}
</style>
