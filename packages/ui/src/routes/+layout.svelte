<script>
	import "../app.css"

	let { children } = $props()
</script>

<div class="flex h-screen items-center justify-center">
    {@render children()}
</div>
