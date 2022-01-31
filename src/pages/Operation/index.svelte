<script lang="ts">
	// components only used here
	import Install from "./Install.svelte"
	import Remove from "./Remove.svelte"

	// components also used outside
	import HomeButton from "../../components/HomeButton.svelte"
	import Spinner from "../../components/Spinner.svelte"

	// svelte stuff
	import { querystring } from "svelte-spa-router"

	// etc
	import { install, remove } from "./logic"
	import store from "./store"

	// types
	import { OperationType } from "./types"

	//
	// variables
	//

	const operationType: OperationType = $querystring.includes("install")
		? OperationType.Install
		: OperationType.Remove

	let wasButtonClicked = false // if the install/remove button was clicked or not
	let spinCog = false

	//
	//
	//

	store.wasButtonClicked.subscribe((value) => {
		wasButtonClicked = value
	})

	store.spinCog.subscribe((value) => {
		spinCog = value
	})
</script>

<!-- Allow user to go back to home until they click the install button -->
{#if !wasButtonClicked}
	<HomeButton />
{/if}

<div class="page">
	<Spinner active={spinCog} />

	{#if !wasButtonClicked}
		<!-- Hide after clicking the button -->
		{#if operationType == OperationType.Install}
			<button on:click|once={() => install()}>Install/Update!</button>
		{:else}
			<button on:click|once={() => remove()}>Remove!</button>
		{/if}
	{:else}
		<!-- Show only when the button is clicked -->
		{#if operationType == OperationType.Install}
			<Install />
		{:else}
			<Remove />
		{/if}
	{/if}
</div>

<style lang="scss">
	@import "./styles/button.scss";

	.page {
		@apply flex flex-col items-center;
	}
</style>
