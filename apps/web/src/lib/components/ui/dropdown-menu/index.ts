import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"

import Content from "./dropdown-menu-content.svelte"
import Item from "./dropdown-menu-item.svelte"
const Root = DropdownMenuPrimitive.Root
const Trigger = DropdownMenuPrimitive.Trigger
const Group = DropdownMenuPrimitive.Group

export {
	Content,
	Content as DropdownMenuContent,
	Group,
	Group as DropdownMenuGroup,
	Item,
	Item as DropdownMenuItem,
	Root,
	//
	Root as DropdownMenu,
	Trigger,
	Trigger as DropdownMenuTrigger,
}
