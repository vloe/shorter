<script lang="ts">
	const title = "404: Page not found"
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
</svelte:head>

<div class="flex h-screen items-center justify-center">
	<h1 class="text-lg">{title}</h1>
</div>
