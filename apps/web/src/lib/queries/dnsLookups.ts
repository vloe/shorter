import type { DnsLookupsParams, DnsLookupsRes } from "$lib/utils/bindings"

import { apiUrl } from "$lib/utils/urls"

export async function search(params: DnsLookupsParams): Promise<DnsLookupsRes> {
	const url = new URL(`${apiUrl().origin}/dns-lookups`)

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
	const data: DnsLookupsRes = await res.json()
	return data
}
