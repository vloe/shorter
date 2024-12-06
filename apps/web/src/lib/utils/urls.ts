import { dev } from "$app/environment"

const API_URL_DEV = "http://localhost:9000"
const API_URL_PROD = "https://ayeqgpklbwm2fj3le6i6ehxkwe0oloez.lambda-url.eu-west-2.on.aws"

export function apiUrl() {
	const apiUrlStr = dev ? API_URL_DEV : API_URL_PROD
	return new URL(apiUrlStr)
}
