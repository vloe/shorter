<script lang="ts">
	import Lockup from "@sh/assets/svgs/lockup.svg"
	import { Btn } from "@sh/ui/src/lib/components/btn"
</script>

<header class="flex h-16 items-center justify-between">
	<a href="/">
		<img alt="lockup" class="h-5" src={Lockup} />
	</a>
	<div class="flex items-center gap-x-2">
		<Btn intent="ghost">test</Btn>
	</div>
</header>
