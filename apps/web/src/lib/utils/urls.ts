import { dev } from "$app/environment"

const API_URL_DEV = "http://localhost:9000"
const API_URL_PROD = "https://wqf5ycrsaonxgabldcousojjta0yafwm.lambda-url.eu-west-2.on.aws/"

export function apiUrl() {
	const apiUrlStr = dev ? API_URL_DEV : API_URL_PROD
	return new URL(apiUrlStr)
}
