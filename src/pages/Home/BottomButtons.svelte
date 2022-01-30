<script lang="ts">
	import FaBook from "svelte-icons/fa/FaBook.svelte"
	import FaGithub from "svelte-icons/fa/FaGithub.svelte"
	import FaDiscord from "svelte-icons/fa/FaDiscord.svelte"
</script>

<div class="small-button-container">
	<a href="https://github.com/War-Brokers-Mods/WBM#usage" target="_blank">
		<FaBook />
	</a>

	<a href="https://github.com/War-Brokers-Mods" target="_blank">
		<FaGithub />
	</a>

	<a href="https://discord.gg/aQqamSCUcS" target="_blank">
		<FaDiscord />
	</a>
</div>

<style lang="scss">
	.small-button-container {
		@apply grid gap-4 grid-cols-3 mt-6;

		a {
			@apply w-6;

			&:hover {
				@apply text-gray-200;
			}

			&:active {
				@apply text-gray-300;
			}
		}
	}
</style>
