<script lang="ts">
	import Theme from "$lib/components/Theme.svelte"
	import Github from "$lib/icons/Github.svelte"
	import Link from "$lib/components/Link.svelte"
</script>

<nav class="flex h-20 items-center justify-between">
	<a href="/" class="text-2xl font-thin">shorter.dev</a>
	<div class="flex items-center space-x-2.5">
		<Theme />
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
