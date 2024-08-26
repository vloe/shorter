import { Command as CommandPrimitive } from "cmdk-sv"

import Root from "./command.svelte"
import Empty from "./command-empty.svelte"
import Group from "./command-group.svelte"
import Item from "./command-item.svelte"
import List from "./command-list.svelte"
import Separator from "./command-separator.svelte"
import Shortcut from "./command-shortcut.svelte"

const Loading = CommandPrimitive.Loading

export {
	Empty,
	Empty as CommandEmpty,
	Group,
	Group as CommandGroup,
	Item,
	Item as CommandItem,
	List,
	List as CommandList,
	Loading,
	Loading as CommandLoading,
	Root,
	//
	Root as Command,
	Separator,
	Separator as CommandSeparator,
	Shortcut,
	Shortcut as CommandShortcut,
}
