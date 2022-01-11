import Home from "./pages/Home/index.svelte"
import Status from "./pages/Status/index.svelte"
import Install from "./pages/Install/index.svelte"
import Update from "./pages/Update/index.svelte"

export default {
	"/": Home,
	"/status": Status,
	"/install": Install,
	"/update": Update,
}
