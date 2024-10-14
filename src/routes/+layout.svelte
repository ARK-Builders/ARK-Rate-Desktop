<script lang="ts">
  import { Button, Input } from "flowbite-svelte";
  import {
    Bell,
    ChartNoAxesColumn,
    Coins,
    PanelLeft,
    Search,
    Settings,
  } from "lucide-svelte";
  import "../app.css";

  let isNavigationDrawerOpen = false;
  const toggleNavigationDrawer = () =>
    (isNavigationDrawerOpen = !isNavigationDrawerOpen);
</script>

<div class="flex box-border min-w-full min-h-screen">
  <aside
    class="h-full transition-all {isNavigationDrawerOpen ? ' w-64' : ' w-16'}"
  >
    {#if isNavigationDrawerOpen}
      <div class="flex flex-col gap-4 py-10 px-4">
        <div class="flex justify-between items-center px-4">
          <img class="size-14" src="/md-logo.png" alt="Ark Rate Logo" />
          <Button
            size="xs"
            outline
            color="light"
            on:click={toggleNavigationDrawer}
          >
            <PanelLeft class="size-5" />
          </Button>
        </div>
        <Input type="search" placeholder="Search" size="md">
          <Search slot="left" class="size-5" />
        </Input>
        <Button
          outline
          size="sm"
          color="primary"
          class="justify-start gap-2 px-3 shadow-none border-none"
        >
          <Coins class="size-5" />
          Quick
        </Button>
        <Button
          outline
          size="sm"
          color="primary"
          class="justify-start gap-2 px-3 shadow-none border-none"
        >
          <ChartNoAxesColumn class="size-5" />
          Portfolios
        </Button>
        <Button
          outline
          size="sm"
          color="primary"
          class="justify-start gap-2 px-3 shadow-none border-none"
        >
          <Bell class="size-5" />
          Alerts
        </Button>
        <Button
          outline
          size="sm"
          color="primary"
          class="justify-start gap-2 px-3 shadow-none border-none"
        >
          <Settings class="size-5" />
          Settings
        </Button>
      </div>
    {:else}
      <div class="flex flex-col gap-4 py-10 px-4 items-center">
        <img class="size-8" src="/sm-logo.png" alt="Ark Rate Logo" />
        <Button
          size="xs"
          outline
          color="light"
          on:click={toggleNavigationDrawer}
        >
          <PanelLeft class="size-4" />
        </Button>
        <Button size="xs" outline color="light">
          <Search class="size-4" />
        </Button>
        <Button size="xs" outline color="primary">
          <Coins class="size-4" />
        </Button>
        <Button size="xs" outline color="primary">
          <ChartNoAxesColumn class="size-4" />
        </Button>
        <Button size="xs" outline color="primary">
          <Bell class="size-4" />
        </Button>
        <Button size="xs" outline color="primary">
          <Settings class="size-4" />
        </Button>
      </div>
    {/if}
  </aside>
  <div class="flex-grow bg-yellow-300">
    <slot />
  </div>
</div>
