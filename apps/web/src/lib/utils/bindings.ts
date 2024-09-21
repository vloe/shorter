/*
 Generated by typeshare 1.8.0
*/

export interface TldInfo {
	category: string;
	manager: string;
}

export interface Domain {
	name: string;
	sld: string;
	tld: string;
	tldWithDot: string;
	tldInfo: TldInfo;
}

export interface DnsLookupParams {
	q: string[];
}

export interface DnsLookupRes {
	lookup: Record<string, boolean>;
}

export interface SearchParams {
	q: string;
}

export interface SearchRes {
	domains: Domain[];
}

