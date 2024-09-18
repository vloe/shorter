import type { SearchParams, SearchRes } from "$lib/utils/bindings"

import { apiUrl } from "$lib/utils/urls"

export async function search(params: SearchParams): Promise<SearchRes> {
	const url = new URL(`${apiUrl().origin}/search`)
	url.searchParams.append("q", params.q)

	const res = await fetch(url.toString(), {
		headers: {
			"Accept": "application/json",
			"Cache-Control": "max-age=300",
			"Content-Type": "application/json",
		},
		method: "GET",
	})

	if (!res.ok) throw new Error(await res.text())
	const data: SearchRes = await res.json()
	return data
}
