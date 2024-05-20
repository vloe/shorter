<script lang="ts">
	import { theme } from "$lib/stores/theme"
	import Button from "@sh/ui/src/lib/components/Button.svelte"

	$: isDark = $theme === "dark"
</script>

<nav class="flex h-20 items-center justify-between">
	<a href="/" class="text-2xl">shorter.dev</a>
	<Button>hallo</Button>
</nav>
