import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"

import Content from "./dropdown-menu-content.svelte"
import Item from "./dropdown-menu-item.svelte"

const Root = DropdownMenuPrimitive.Root
const Trigger = DropdownMenuPrimitive.Trigger

export {
	Content,
	Content as DropdownMenuContent,
	Item as DropdownMenuItem,
	Item,
	Root as DropdownMenu,
	Root,
	Trigger as DropdownMenuTrigger,
	Trigger,
}
