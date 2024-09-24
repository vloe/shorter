import { Command as CommandPrimitive } from "cmdk-sv"

import Root from "./command.svelte"
import Group from "./command-group.svelte"
import Input from "./command-input.svelte"
import Item from "./command-item.svelte"

const Loading = CommandPrimitive.Loading

export {
	Group,
	Input,
	Item,
	Loading,
	Root,
	//
	Root as Command,
}
