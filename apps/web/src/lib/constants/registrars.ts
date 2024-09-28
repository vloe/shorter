import type { Component } from "svelte"

import { Cloudflare } from "$lib/components/icons/cloudflare"
import { Godaddy } from "$lib/components/icons/godaddy"
import { Hostgator } from "$lib/components/icons/hostgator"
import { Hover } from "$lib/components/icons/hover"
import { Namecheap } from "$lib/components/icons/namecheap"
import { Namesilo } from "$lib/components/icons/namesilo"
import { Porkbun } from "$lib/components/icons/porkbun"

type Registrar = {
	buyLink?: string
	icon: Component
	name: string
	site: string
}

export const registrars: Array<Registrar> = [
	{
		buyLink: "https://porkbun.com/checkout/search?q=",
		icon: Porkbun,
		name: "porkbun",
		site: "https://porkbun.com",
	},
	{
		icon: Cloudflare,
		name: "cloudflare (login)",
		site: "https://dash.cloudflare.com/?account=domains/register",
	},
	{
		buyLink: "https://namecheap.com/domains/registration/results/?domain=",
		icon: Namecheap,
		name: "namecheap",
		site: "https://namecheap.com",
	},
	{
		buyLink: "https://godaddy.com/domainsearch/find?domainToCheck=",
		icon: Godaddy,
		name: "godaddy",
		site: "https://godaddy.com",
	},
	{
		buyLink: "https://namesilo.com/domain/search-domains/?query=",
		icon: Namesilo,
		name: "namesilo",
		site: "https://namesilo.com",
	},
	{
		buyLink: "https://hostgator.com/registration/?search=",
		icon: Hostgator,
		name: "hostgator",
		site: "https://hostgator.com",
	},
	{
		buyLink: "https://hover.com/domains/results?q=",
		icon: Hover,
		name: "hover",
		site: "https://hover.com",
	},
]
