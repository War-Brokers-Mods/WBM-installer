<script lang="ts">
	import Notifications from "svelte-notifications"

	import Router from "svelte-spa-router"
	import routes from "./routes"
</script>

<Notifications>
	<Router {routes} />
</Notifications>
