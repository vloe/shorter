<script lang="ts">
	import init, { WebGLRenderer } from "../../../../webgl/pkg/sh_webgl"

	let canvas: HTMLCanvasElement
	let renderer: WebGLRenderer
	let animationId: number
	let cursorX = $state(0)
	let cursorY = $state(0)
	let scrollY = $state(0)

	$effect(() => {
		async function setupWebGL() {
			await init()
			canvas.width = window.innerWidth
			canvas.height = window.innerHeight
			renderer = new WebGLRenderer(canvas)
			animate()
		}

		setupWebGL()
	})

	function animate(time = 0) {
		console.log(time)
		if (renderer) {
			const normalizedCursorX = (cursorX / window.innerWidth) * 2 - 1
			const normalizedCursorY = -(cursorY / window.innerHeight) * 2 + 1
			const normalizedScrollY =
				scrollY / (document.documentElement.scrollHeight - window.innerHeight)

			renderer.render(time * 0.001, normalizedCursorX, normalizedCursorY, normalizedScrollY)
		}
		animationId = requestAnimationFrame(animate)
	}

	function handleMouseMove(event: MouseEvent) {
		cursorX = event.clientX
		cursorY = event.clientY
	}

	function handleScroll() {
		scrollY = window.scrollY
	}

	$effect(() => {
		function handleResize() {
			if (canvas && renderer) {
				canvas.width = window.innerWidth
				canvas.height = window.innerHeight
				renderer.resize(canvas.width, canvas.height)
			}
		}

		window.addEventListener("resize", handleResize)
		window.addEventListener("mousemove", handleMouseMove)
		window.addEventListener("scroll", handleScroll)

		return () => {
			window.removeEventListener("resize", handleResize)
			window.removeEventListener("mousemove", handleMouseMove)
			window.removeEventListener("scroll", handleScroll)
			cancelAnimationFrame(animationId)
		}
	})
</script>

<canvas bind:this={canvas} class="fixed left-0 top-0 -z-10 h-full w-full"></canvas>
