import { dev } from "$app/environment"

export const API_URL = dev ? "http://localhost:8787" : "https://api.shorter.dev"
