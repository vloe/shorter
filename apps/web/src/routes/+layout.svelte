<script lang="ts">
	import type { Snippet } from "svelte"

	import "../app.css"

	type $Props = {
		children: Snippet
	}

	let { children }: $Props = $props()
</script>

<div class="container mx-auto scroll-smooth px-6 lg:px-14">
	{@render children()}
</div>
