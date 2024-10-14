<script lang="ts">
	import { Button, Input } from 'flowbite-svelte';
	import { Bell, ChartNoAxesColumn, Coins, PanelLeft, Search, Settings } from 'lucide-svelte';
	import '../app.css';

	let isNavigationDrawerOpen = true;
	const toggleNavigationDrawer = () => (isNavigationDrawerOpen = !isNavigationDrawerOpen);
</script>

<div class="box-border flex min-h-screen min-w-full">
	<aside class="border-r transition-all {isNavigationDrawerOpen ? ' w-64' : ' w-16'}">
		{#if isNavigationDrawerOpen}
			<div class="flex flex-col gap-4 px-4 py-10">
				<div class="flex items-center justify-between px-4">
					<img
						class="size-14"
						src="/md-logo.png"
						alt="Ark Rate Logo"
					/>
					<Button
						outline
						size="xs"
						color="light"
						on:click={toggleNavigationDrawer}
					>
						<PanelLeft class="size-5" />
					</Button>
				</div>
				<Input
					size="md"
					type="search"
					placeholder="Search"
				>
					<Search
						slot="left"
						class="size-5"
					/>
				</Input>
				<Button
					outline
					size="sm"
					color="primary"
					class="justify-start gap-2 border-none px-3 shadow-none"
				>
					<Coins class="size-5" />
					Quick
				</Button>
				<Button
					outline
					size="sm"
					color="primary"
					class="justify-start gap-2 border-none px-3 shadow-none"
				>
					<ChartNoAxesColumn class="size-5" />
					Portfolios
				</Button>
				<Button
					outline
					size="sm"
					color="primary"
					class="justify-start gap-2 border-none px-3 shadow-none"
				>
					<Bell class="size-5" />
					Alerts
				</Button>
				<Button
					outline
					size="sm"
					color="primary"
					class="justify-start gap-2 border-none px-3 shadow-none"
				>
					<Settings class="size-5" />
					Settings
				</Button>
			</div>
		{:else}
			<div class="flex flex-col items-center gap-4 px-4 py-10">
				<img
					class="size-8"
					src="/sm-logo.png"
					alt="Ark Rate Logo"
				/>
				<Button
					outline
					size="xs"
					color="light"
					on:click={toggleNavigationDrawer}
				>
					<PanelLeft class="size-4" />
				</Button>
				<Button
					outline
					size="xs"
					color="light"
				>
					<Search class="size-4" />
				</Button>
				<Button
					outline
					size="xs"
					color="primary"
				>
					<Coins class="size-4" />
				</Button>
				<Button
					outline
					size="xs"
					color="primary"
				>
					<ChartNoAxesColumn class="size-4" />
				</Button>
				<Button
					outline
					size="xs"
					color="primary"
				>
					<Bell class="size-4" />
				</Button>
				<Button
					outline
					size="xs"
					color="primary"
				>
					<Settings class="size-4" />
				</Button>
			</div>
		{/if}
	</aside>
	<div class="flex-grow">
		<slot />
	</div>
</div>
