<script lang="ts">
</script>

<div class="spinner">
	<img alt="WBM back" src="/img/WBM-back.png" />
	<img alt="WBM left gear" class="rotateL" src="/img/WBM-Lgear.png" />
	<img alt="WBM right gear" class="rotateR" src="/img/WBM-Rgear.png" />
	<img alt="WBM ftont" src="/img/WBM-front.png" />
</div>

<style lang="scss">
	.spinner {
		@apply flex w-[128px] h-[128px];

		img {
			@apply absolute w-[128px] h-[128px];
		}

		.rotateL {
			animation: spin 1.5s linear infinite;
			transform-origin: 34.5px 71.5px;
		}

		.rotateR {
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
