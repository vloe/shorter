<script lang="ts">
	import { theme } from "$lib/stores/theme"
	import Button from "$lib/components/Button.svelte"
	import Moon from "$lib/icons/Moon.svelte"
	import Sun from "$lib/icons/Sun.svelte"
	import Github from "$lib/icons/Github.svelte"
	import Link from "$lib/components/Link.svelte"

	$: isDark = $theme === "dark"
</script>

<nav class="flex h-[72px] items-center justify-between">
	<a href="/" class="text-2xl font-thin">shorter.dev</a>
	<div class="flex items-center space-x-2.5">
		<Button variant="outline" size="icon" on:click={() => theme.set(isDark ? "light" : "dark")}>
			<Moon class="absolute rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
			<Sun class="rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
		</Button>
		<Link
			href="https://github.com/vloe/shorter"
			target="_blank"
			rel="noreferrer"
			variant="outline"
			size="icon"
		>
			<Github />
		</Link>
	</div>
</nav>
