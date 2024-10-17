import type { DnsLookupParams, DnsLookupRes } from "$lib/types/bindings"

import { apiUrl } from "$lib/utils/urls"

export async function dnsLookup(params: DnsLookupParams): Promise<DnsLookupRes> {
	const url = new URL(`${apiUrl().origin}/dns-lookup`)

	for (let i = 0; i < params.q.length; i++) {
		url.searchParams.append("q", params.q[i])
	}

	const res = await fetch(url.toString(), {
		headers: {
			"Accept": "application/json",
			"Cache-Control": "max-age=300",
			"Content-Type": "application/json",
		},
		method: "GET",
	})

	if (!res.ok) throw new Error(await res.text())
	const data: DnsLookupRes = await res.json()
	return data
}
