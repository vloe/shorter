				import worker, * as OTHER_EXPORTS from "/Users/vloe/projects/shorter/apps/server/build/worker/shim.mjs";
				import * as __MIDDLEWARE_0__ from "/Users/vloe/projects/shorter/node_modules/.pnpm/wrangler@3.57.1_@cloudflare+workers-types@4.20240529.0/node_modules/wrangler/templates/middleware/middleware-ensure-req-body-drained.ts";
import * as __MIDDLEWARE_1__ from "/Users/vloe/projects/shorter/node_modules/.pnpm/wrangler@3.57.1_@cloudflare+workers-types@4.20240529.0/node_modules/wrangler/templates/middleware/middleware-miniflare3-json-error.ts";
				
				worker.middleware = [
					__MIDDLEWARE_0__.default,__MIDDLEWARE_1__.default,
					...(worker.middleware ?? []),
				].filter(Boolean);
				
				export * from "/Users/vloe/projects/shorter/apps/server/build/worker/shim.mjs";
				export default worker;