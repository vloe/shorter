import { createDropdownMenu } from "@melt-ui/svelte"
import { getContext, setContext } from "svelte"

export const setCtx = () => {
	const dropdownMenu = createDropdownMenu()
	return setContext("menu", dropdownMenu)
}

export const getCtx = () => {
	type DropdownMenu = ReturnType<typeof createDropdownMenu>
	return getContext<DropdownMenu>("menu")
}
