<script lang="ts">
	import { ArrowLink } from "$lib/components/arrow-link"
	import { Link } from "$lib/components/link"
	import { webUrl } from "$lib/utils/webUrl"
	import Lockup from "@sh/assets/svgs/lockup.svg"
	import Logomark from "@sh/assets/svgs/logomark.svg"
</script>

<header class="flex h-16 items-center justify-between lg:h-20">
	<a href="/">
		<img alt="lockup" class="hidden h-7 lg:flex" src={Lockup} />
		<img alt="logomark" class="flex h-7 lg:hidden" src={Logomark} />
	</a>
	<div class="flex items-center gap-x-1.5">
		<ArrowLink class="h-8 rounded-lg duration-200 ease-in-out" href={`${webUrl}/register`}>
			start now
		</ArrowLink>
		<Link class="duration-200 ease-in-out" href={`${webUrl}/login`} intent="secondary">
			login
		</Link>
	</div>
</header>
