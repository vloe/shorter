/*
 Generated by typeshare 1.11.0
*/

export interface DnsLookupParams {
	q: string
}

export interface DnsLookupRes {
	available: boolean
}

export interface Tld {
	name: string
	category: string
	manager: string
}

export interface Domain {
	name: string
	tld: Tld
}

export interface FeedbackPayload {
	msg: string
}

export interface FeedbackRes {
	ok: boolean
}

export interface SearchParams {
	q: string
}

export interface SearchRes {
	domains: Domain[]
}
