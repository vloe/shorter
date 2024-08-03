<script lang="ts">
	import init, { WebGLRenderer } from "../../../../webgl/pkg/sh_webgl"

	let canvas: HTMLCanvasElement
	let renderer: WebGLRenderer

	let animationId = $state(0)
	let cursorX = $state(0)
	let cursorY = $state(0)
	let innerWidth = $state(0)
	let innerHeight = $state(0)
	let scrollY = $state(0)

	$effect(() => {
		async function setupWebGL() {
			await init()
			canvas.width = innerWidth
			canvas.height = innerHeight
			renderer = new WebGLRenderer(canvas)
			animate()
		}

		setupWebGL()
	})

	function animate(time = 0) {
		if (renderer) {
			let normalizedCursorX = (cursorX / innerWidth) * 2 - 1
			let normalizedCursorY = -(cursorY / innerHeight) * 2 + 1
			let normalizedScrollY = scrollY / (document.documentElement.scrollHeight - innerHeight)
			renderer.render(time * 0.001, normalizedCursorX, normalizedCursorY, normalizedScrollY)
		}
		animationId = requestAnimationFrame(animate)
	}

	function handleMouseMove(event: MouseEvent) {
		cursorX = event.clientX
		cursorY = event.clientY
	}

	$effect(() => {
		function handleResize() {
			if (canvas && renderer) {
				canvas.width = innerWidth
				canvas.height = innerHeight
				renderer.resize(canvas.width, canvas.height)
			}
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

<svelte:window bind:scrollY bind:innerWidth bind:innerHeight />
<canvas bind:this={canvas} class="fixed left-0 top-0 -z-10 h-full w-full"></canvas>
