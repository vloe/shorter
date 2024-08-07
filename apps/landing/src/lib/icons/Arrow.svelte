<script lang="ts">
	import { cx } from "$lib/utils/cva.config"

	type $Props = {
		class?: string
	}

	let { class: className }: $Props = $props()
</script>

<svg
<<<<<<< HEAD
	class={cx("size-[10px] stroke-[1.5px]", className)}
	fill="none"
	stroke="currentColor"
	viewBox="0 0 10 10"
	xmlns="http://www.w3.org/2000/svg"
>
	<path class="opacity-0 transition-opacity group-hover:opacity-100" d="M0 5h7" />
	<path class="transition-all ease-in-out group-hover:translate-x-[2px]" d="M1 1l4 4-4 4" />
=======
	xmlns="http://www.w3.org/2000/svg"
	fill="none"
	viewBox="0 0 10 10"
	stroke="currentColor"
	class={cx("size-[10px] stroke-[1.5px]", className)}
>
	<path d="M0 5h7" class="opacity-0 transition-opacity group-hover:opacity-100" />
	<path d="M1 1l4 4-4 4" class="transition-all ease-in-out group-hover:translate-x-[2px]" />
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c
</svg>
