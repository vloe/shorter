<script lang="ts">
	import init, { WebGLRenderer } from "../../../../webgl/pkg/sh_webgl"

	let canvas: HTMLCanvasElement
	let renderer: WebGLRenderer

	$effect(() => {
		async function setupWebGL() {
			await init()
			canvas.width = window.innerWidth
			canvas.height = window.innerHeight
			renderer = new WebGLRenderer(canvas)
			renderer.render()
		}

		setupWebGL()
	})
</script>

<canvas bind:this={canvas} class="fixed left-0 top-0 -z-10 h-full w-full"></canvas>
