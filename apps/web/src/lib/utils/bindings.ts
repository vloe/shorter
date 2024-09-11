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
	isRegistered: boolean;
}

export interface DnsLookupParams {
	q: string;
}

export interface DnsLookupRes {
	domain: string;
	available: boolean;
}

export interface SearchParams {
	q: string;
}

export interface SearchRes {
	domains: Domain[];
}

