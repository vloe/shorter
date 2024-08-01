import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"
import DropdownMenuItem from "./dropdown-menu-item.svelte"
import DropdownMenuContent from "./dropdown-menu-content.svelte"

const DropdownMenu = DropdownMenuPrimitive.Root
const DropdownMenuTrigger = DropdownMenuPrimitive.Trigger

export { DropdownMenu, DropdownMenuItem, DropdownMenuContent, DropdownMenuTrigger }
