const API_URL_DEV = "http://localhost:9000"
const API_URL_PROD = "https://6b4ix4gkclnatclhzs7xaancki0ablqs.lambda-url.eu-west-2.on.aws"
const WEB_URL_DEV = "http://localhost:3001"
const WEB_URL_PROD = "https://dashboard.shorter.dev"

export const apiUrl = (dev: boolean): string => {
	return dev ? API_URL_DEV : API_URL_PROD
}

export const webUrl = (dev: boolean): string => {
	return dev ? WEB_URL_DEV : WEB_URL_PROD
}
