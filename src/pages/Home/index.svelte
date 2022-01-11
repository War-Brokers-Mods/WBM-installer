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
		@apply flex flex-col items-center;

		img {
			/* prevent button position flickering */
			@apply min-h-[128px];
		}
	}
</style>
