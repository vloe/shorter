<script>
	import "../app.css"

	let { children } = $props()
</script>

<div class="container max-w-screen-xl px-8">
	{@render children()}
</div>
