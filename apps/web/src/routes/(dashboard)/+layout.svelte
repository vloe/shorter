<script lang="ts">
	import type { Snippet } from "svelte"

	import { Sidebar } from "$lib/components/sidebar"

	type $Props = {
		children: Snippet
	}

	let { children }: $Props = $props()
</script>

<div class="flex h-full min-h-[100dvh]">
	<Sidebar />
	{@render children()}
</div>
