import { dev } from "$app/environment"

export const prerender = true // ssg
export const csr = dev // no js is shipped to the client (enabled in dev for hmr)
