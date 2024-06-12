import { dev } from "$app/environment"

export const apiUrl = dev ? "http://localhost:8787" : "https://api.shorter.dev"
