<script lang="ts">
	import ActionButtons from "./ActionButtons.svelte"
	import BottomButtons from "./BottomButtons.svelte"
</script>

<div class="home">
	<img alt="WBM icon" src="/img/logo@128x128.png" />

	<ActionButtons />
	<BottomButtons />
</div>

<style lang="scss">
	.home {
		@apply flex flex-col justify-center items-center w-full h-full;

		img {
			@apply place-self-center
			m-2
			/* prevent button position flickering */
			min-h-[128px];
		}
	}
</style>
