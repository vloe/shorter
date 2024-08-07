import type { TransitionConfig } from "svelte/transition"

import { cubicOut } from "svelte/easing"

type FlyAndScaleParams = {
	duration?: number
	start?: number
	x?: number
	y?: number
}

export function styleToString(style: Record<string, number | string | undefined>): string {
	return Object.keys(style).reduce((str, key) => {
		if (style[key] === undefined) return str
		return `${str}${key}:${style[key]};`
	}, "")
}

export function flyAndScale(
	node: Element,
<<<<<<< HEAD:apps/landing/src/lib/utils/flyAndScale.ts
	params: FlyAndScaleParams = { duration: 150, start: 0.95, x: 0, y: -8 },
=======
	params: FlyAndScaleParams = { y: -8, x: 0, start: 0.95, duration: 150 },
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c:packages/utils/src/flyAndScale.ts
): TransitionConfig {
	const style = getComputedStyle(node)
	const transform = style.transform === "none" ? "" : style.transform

	const scaleConversion = (
		valueA: number,
		scaleA: [number, number],
		scaleB: [number, number],
	) => {
		const [minA, maxA] = scaleA
		const [minB, maxB] = scaleB

		const percentage = (valueA - minA) / (maxA - minA)
		const valueB = percentage * (maxB - minB) + minB

		return valueB
	}

	return {
<<<<<<< HEAD:apps/landing/src/lib/utils/flyAndScale.ts
=======
		duration: 100,
		delay: 0,
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c:packages/utils/src/flyAndScale.ts
		css: (t) => {
			const y = scaleConversion(t, [0, 1], [params.y ?? 5, 0])
			const x = scaleConversion(t, [0, 1], [params.x ?? 0, 0])
			const scale = scaleConversion(t, [0, 1], [params.start ?? 0.95, 1])

			return styleToString({
				opacity: t,
				transform: `${transform} translate3d(${x}px, ${y}px, 0) scale(${scale})`,
			})
		},
		delay: 0,
		duration: 100,
		easing: cubicOut,
	}
}
