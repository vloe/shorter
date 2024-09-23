import { Command as CommandPrimitive } from "cmdk-sv"

import Root from "./command.svelte"
import Empty from "./command-empty.svelte"
import Input from "./command-input.svelte"
import Item from "./command-item.svelte"

const Loading = CommandPrimitive.Loading

export {
	Empty,
	Empty as CommandEmpty,
	Input,
	Input as CommandInput,
	Item,
	Item as CommandItem,
	Loading,
	Loading as CommandLoading,
	Root,
	//
	Root as Command,
}
