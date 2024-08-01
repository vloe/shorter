<script lang="ts">
	import Logomark from "$lib/assets/svgs/white-logomark.svg"
	import Arrow from "$lib/icons/Arrow.svelte"
	import { Btn } from "$lib/ui/btn"
</script>

<header class="flex h-16 flex-row items-center justify-between">
	<a href="/">
		<img class="h-7" src={Logomark} alt="logomark" />
	</a>
	<div class="flex items-center gap-x-2">
		<Btn class="group gap-x-[5px] duration-200 ease-in-out">
			start now
			<Arrow />
		</Btn>
		<Btn intent="secondary" class="duration-200 ease-in-out">login</Btn>
	</div>
</header>
