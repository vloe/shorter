<script lang="ts">
	import { Btn } from "@sh/ui/src/lib/components/btn"
	import * as Card from "@sh/ui/src/lib/components/card"
	import Github from "@sh/ui/src/lib/icons/Github.svelte"
	import Google from "@sh/ui/src/lib/icons/Google.svelte"
</script>

<Card.Header>
	<Card.Title class="text-center text-2xl">create your shorter account</Card.Title>
</Card.Header>
<Card.Content class="flex flex-col items-center gap-y-4">
	<Btn class="text-md h-12 w-full max-w-sm" intent="secondary">
		<Github />
		<span class="ml-2.5">continue with github</span>
	</Btn>
	<Btn class="text-md h-12 w-full max-w-sm">
		<Google />
		<span class="ml-2.5">continue with google</span>
	</Btn>
</Card.Content>
<Card.Footer>
	<p class="w-full text-center text-sm">
		<span class="text-primary/85">already have an account?</span>
		<a class="text-indigo-400" href="/login">login</a>
	</p>
</Card.Footer>
