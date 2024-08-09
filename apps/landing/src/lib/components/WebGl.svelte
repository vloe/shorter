<script lang="ts">
	import init, { WebGLRenderer } from "../../../../webgl/pkg/sh_webgl"

	let renderer: null | WebGLRenderer = $state(null)
	let canvas: HTMLCanvasElement | null = $state(null)
	let animationId = $state(0)
	let cursorX = $state(0)
	let cursorY = $state(0)
	let innerWidth = $state(0)
	let innerHeight = $state(0)

	$effect(() => {
		function animate(time = 0) {
			if (renderer) {
				let normalizedCursorX = cursorX / innerWidth
				let normalizedCursorY = 1.0 - cursorY / innerHeight
				renderer.render(time * 0.001, normalizedCursorX, normalizedCursorY)
			}
			animationId = requestAnimationFrame(animate)
		}
		async function setupWebGL() {
			if (!canvas) return
			await init()
			canvas.width = innerWidth
			canvas.height = innerHeight
			renderer = new WebGLRenderer(canvas)
			animate()
		}

		setupWebGL()
	})

	$effect(() => {
		function handleResize() {
			if (canvas && renderer) {
				canvas.width = innerWidth
				canvas.height = innerHeight
				renderer.resize(canvas.width, canvas.height)
			}
		}
		function handleMouseMove(event: MouseEvent) {
			cursorX = event.clientX
			cursorY = event.clientY
		}

		window.addEventListener("resize", handleResize)
		window.addEventListener("mousemove", handleMouseMove)

		return () => {
			window.removeEventListener("resize", handleResize)
			window.removeEventListener("mousemove", handleMouseMove)
			cancelAnimationFrame(animationId)
		}
	})
</script>

<svelte:window bind:innerHeight bind:innerWidth />
<canvas bind:this={canvas} class="absolute left-0 top-0 -z-50 h-full w-full"></canvas>
