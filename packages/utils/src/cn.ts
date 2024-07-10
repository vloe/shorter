import { twMerge } from "tailwind-merge"
import { clsx } from "clsx"
import type { ClassValue } from "clsx"

export const cn = (...args: ClassValue[]) => {
	return twMerge(clsx(...args))
}
