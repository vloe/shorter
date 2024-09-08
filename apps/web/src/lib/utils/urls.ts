import { dev } from "$app/environment"

const API_URL_DEV = "http://localhost:9000"
const API_URL_PROD = "https://6b4ix4gkclnatclhzs7xaancki0ablqs.lambda-url.eu-west-2.on.aws"

export const apiUrl = dev ? API_URL_DEV : API_URL_PROD
