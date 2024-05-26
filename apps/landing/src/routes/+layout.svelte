<script>
	import "../app.css"
    import Header from "$lib/components/Header.svelte"

	let { children } = $props()
</script>
	
<div>
    <Header />
    {@render children()}
</div>
