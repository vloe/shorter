<script lang="ts">
	import { page } from "$app/stores"
</script>

<div class="flex h-screen items-center justify-center">
	<h1 class="text-lg text-white/90">
		{$page.status}: {$page.error?.message?.toLowerCase()}
	</h1>
</div>
