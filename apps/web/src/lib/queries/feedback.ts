import type { FeedbackPayload, FeedbackRes } from "$lib/utils/bindings"

import { apiUrl } from "$lib/utils/urls"

export async function feedback(payload: FeedbackPayload): Promise<FeedbackRes> {
	const url = new URL(`${apiUrl().origin}/feedback`)

	const res = await fetch(url.toString(), {
		body: JSON.stringify(payload),
		headers: {
			"Accept": "application/json",
			"Content-Type": "application/json",
		},
		method: "POST",
	})

	if (!res.ok) throw new Error(await res.text())
	const data: FeedbackRes = await res.json()
	return data
}
