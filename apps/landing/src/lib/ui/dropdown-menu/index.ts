import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"

import DropdownMenuContent from "./dropdown-menu-content.svelte"
import DropdownMenuItem from "./dropdown-menu-item.svelte"

const DropdownMenu = DropdownMenuPrimitive.Root
const DropdownMenuTrigger = DropdownMenuPrimitive.Trigger

export { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger }
