import { Bookmarks } from "@sh/ui/src/lib/components/icons/bookmarks"
import { Links } from "@sh/ui/src/lib/components/icons/links"
import { Search } from "@sh/ui/src/lib/components/icons/search"

import Sidebar from "./sidebar.svelte"

const navLinks = [
	{ href: "/bookmarks", icon: Bookmarks, name: "bookmarks" },
	{ href: "/", icon: Search, name: "search" },
	{ href: "/links", icon: Links, name: "links" },
]

export { navLinks, Sidebar }
