<script lang="ts">
  import { page } from '$app/stores';
  import Toast from '$lib/ui/global/components/toast/Toast.svelte';
  import { toasts } from '$lib/ui/global/stores/toastStore';
  import { Button, Input } from 'flowbite-svelte';
  import { Bell, ChartNoAxesColumn, Coins, PanelLeft, Search, Settings } from 'lucide-svelte';
  import '../app.css';

  let isNavigationDrawerOpen = true;

  const toggleNavigationDrawer = () => (isNavigationDrawerOpen = !isNavigationDrawerOpen);
</script>

<div class="center pointer-events-none fixed z-50 box-border flex w-screen flex-col items-center justify-center gap-4">
  {#each $toasts as toast}
    <Toast
      id={toast.id}
      type={toast.type}
      message={toast.message}
    />
  {/each}
</div>

<div class="flex min-h-screen min-w-full">
  <aside class="border-r transition-all {isNavigationDrawerOpen ? 'w-64 min-w-48' : 'w-16'}">
    {#if isNavigationDrawerOpen}
      <div class="flex flex-col gap-6 px-4 py-10">
        <div class="flex items-center justify-between pl-4">
          <img
            class="size-14"
            alt="Ark Rate Logo"
            src="/images/ark/md-logo.png"
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
        <div class="flex flex-col gap-3">
          <Button
            outline
            href="/"
            size="sm"
            color="primary"
            checked={$page.url.pathname === '/'}
            class="justify-start gap-1 border-none px-3 shadow-none"
          >
            <Coins class="size-5" />
            Quick
          </Button>
          <Button
            outline
            size="sm"
            color="primary"
            href="/portfolios"
            checked={$page.url.pathname.startsWith('/portfolios')}
            class="justify-start gap-1 border-none px-3 shadow-none"
          >
            <ChartNoAxesColumn class="size-5" />
            Portfolios
          </Button>
          <Button
            outline
            size="sm"
            color="primary"
            href="/alerts"
            checked={$page.url.pathname.startsWith('/alerts')}
            class="justify-start gap-1 border-none px-3 shadow-none"
          >
            <Bell class="size-5" />
            Alerts
          </Button>
          <Button
            outline
            size="sm"
            color="primary"
            href="/settings"
            checked={$page.url.pathname.startsWith('/settings')}
            class="justify-start gap-1 border-none px-3 shadow-none"
          >
            <Settings class="size-5" />
            Settings
          </Button>
        </div>
      </div>
    {:else}
      <div class="flex flex-col items-center gap-4 px-4 py-10">
        <img
          class="size-8"
          alt="Ark Rate Logo"
          src="/images/ark/sm-logo.png"
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
          class="shadow-none"
          checked={$page.url.pathname === '/'}
        >
          <Coins class="size-4" />
        </Button>
        <Button
          outline
          size="xs"
          color="primary"
          class="shadow-none"
          checked={$page.url.pathname.startsWith('/portfolios')}
        >
          <ChartNoAxesColumn class="size-4" />
        </Button>
        <Button
          outline
          size="xs"
          color="primary"
          class="shadow-none"
          checked={$page.url.pathname.startsWith('/alerts')}
        >
          <Bell class="size-4" />
        </Button>
        <Button
          outline
          size="xs"
          color="primary"
          class="shadow-none"
          checked={$page.url.pathname.startsWith('/settings')}
        >
          <Settings class="size-4" />
        </Button>
      </div>
    {/if}
  </aside>
  <div class="relative flex-grow">
    <slot />
  </div>
</div>
