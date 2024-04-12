<script lang="ts">
	import { theme } from "$lib/stores/theme"
	import Button from "$lib/components/Button.svelte"
	import Moon from "$lib/icons/Moon.svelte"
	import Sun from "$lib/icons/Sun.svelte"

	$: isDark = $theme === "dark"
</script>

<Button variant="outline" size="icon" on:click={() => theme.set(isDark ? "light" : "dark")}>
	<Moon class="absolute rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
	<Sun class="rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
</Button>
