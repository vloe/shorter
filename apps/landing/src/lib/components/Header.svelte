<script lang="ts">
	import Logomark from "$lib/assets/svgs/white-logomark.svg"
	import Btn from "$lib/components/Btn.svelte"
	import Arrow from "$lib/icons/Arrow.svelte"
</script>

<header class="flex h-20 flex-row items-center justify-between">
	<a href="/">
		<img class="h-6" src={Logomark} alt="logomark" />
	</a>
	<div class="flex items-center gap-x-2">
		<Btn class="group gap-x-[5px] transition-colors duration-200 ease-in-out">
			start now
			<Arrow />
		</Btn>
		<Btn intent="secondary" href="/login" class="transition-colors duration-200 ease-in-out">
			login
		</Btn>
	</div>
</header>
