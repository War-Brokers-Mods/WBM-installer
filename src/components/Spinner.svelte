<script lang="ts">
	export let active: boolean
</script>

<div class="spinner">
	<img alt="WBM back" src="/img/WBM-back.png" />
	<img
		alt="WBM left gear"
		class="rotateL {active && 'activated'}"
		src="/img/WBM-Lgear.png"
	/>
	<img
		alt="WBM right gear"
		class="rotateR {active && 'activated'}"
		src="/img/WBM-Rgear.png"
	/>
	<img alt="WBM ftont" src="/img/WBM-front.png" />
</div>

<style lang="scss">
	.spinner {
		@apply flex w-[128px] h-[128px];

		img {
			@apply absolute w-[128px] h-[128px];
		}

		.rotateL.activated {
			animation: spin 1.5s linear infinite;
			transform-origin: 34.5px 71.5px;
		}

		.rotateR.activated {
			animation: spin 3s linear infinite;
			transform-origin: 70px 52px;
		}

		@keyframes spin {
			to {
				transform: rotate(360deg);
			}
		}
	}
</style>
