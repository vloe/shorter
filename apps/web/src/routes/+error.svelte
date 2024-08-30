<script lang="ts">
	import { page } from "$app/stores"
</script>

<div class="flex h-screen items-center justify-center">
	<h1 class="text-lg">{$page.status}: {$page.error?.message}</h1>
</div>
