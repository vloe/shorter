<script lang="ts">
	import { AsciiArt } from "$lib/components/ui/ascii-art"
	import { asciiArtPos } from "$lib/constants/asciiArtPos"
</script>

{#each asciiArtPos as { rotation, scale, x, y }}
	<div
		class="absolute -z-50"
		style="left: {x}%; top: {y}%; transform: rotate({rotation}deg) scale({scale});"
	>
		<AsciiArt class="text-[1px]" />
	</div>
{/each}
