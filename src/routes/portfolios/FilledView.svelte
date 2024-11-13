<script lang="ts">
  import type { ViewPortfoliosResponse } from '$lib/business/interactors/view_portfolios/ViewPortfoliosResponse';
  import ContextMenu from '$lib/ui/global/components/context_menu/ContextMenu.svelte';
  import { Button, Heading, TabItem, Tabs } from 'flowbite-svelte';
  import { ArrowDown, ArrowUp, ChartNoAxesColumn, Pencil, Plus, Trash } from 'lucide-svelte';

  type Tag = ViewPortfoliosResponse['tags'][0];
  type Portfolio = ViewPortfoliosResponse['portfolios'][0];

  export let untaggedPortfolios: Portfolio[];
  export let groupedPortfolios: Map<Tag, Portfolio[]>;

  export let onStorePortfoliosOpen: () => void;
  export let onDeleteAssetOpen: (portfolio: Portfolio) => void;
  export let onUpdatePortfolioOpen: (portfolio: Portfolio) => void;

  let portfolioContextMenu:
    | {
        id: string;
        x: number;
        y: number;
      }
    | undefined;

  const getTotalUSDValue = (portfolios: Portfolio[]): number => {
    return portfolios.map((p) => p.asset.usd_value * p.asset.quantity).reduce((acc, current) => acc + current);
  };

  const getTotalFluctuation = (portfolios: Portfolio[]): number => {
    const totalUSDValue = getTotalUSDValue(portfolios);
    const weightedFluctuation = portfolios
      .map((p) => p.fluctuation * p.asset.quantity)
      .reduce((acc, current) => acc + current);
    return weightedFluctuation / totalUSDValue;
  };

  $: portfolios = [untaggedPortfolios, ...groupedPortfolios.values()].flat();
  $: totalFluctuation = getTotalFluctuation(portfolios);
</script>

<svelte:window on:click={() => (portfolioContextMenu = undefined)} />

<div class="mb-16 flex items-center justify-between gap-12">
  <div class="flex gap-4">
    <ChartNoAxesColumn class="size-16 text-green-500" />
    <div class="flex flex-col gap-1">
      <Heading tag="h4">Track Your Assets</Heading>
      <p class="max-w-2xl text-wrap text-gray-500">
        Add currencies manualy to track and compare. Your portfolio helps you monitor and manage your favorite or
        frequently used currencies.
      </p>
    </div>
  </div>
  <Button
    size="md"
    color="primary"
    class="flex gap-1"
    on:click={onStorePortfoliosOpen}
  >
    <Plus />
    New Asset
  </Button>
</div>
<Tabs
  tabStyle="underline"
  contentClass=""
>
  <TabItem
    open
    title="All"
  >
    <div class="flex flex-col">
      <div class="flex flex-col items-start gap-2 border-b px-2 py-10">
        <p class="text-gray-500 dark:text-gray-400">Total Assets</p>
        <div class="flex gap-0.5 align-top">
          <p class="text-2xl">$</p>
          <p class="text-4xl">{getTotalUSDValue(portfolios).toLocaleString()}</p>
          {#if totalFluctuation < 0}
            <div class="flex text-red-500">
              <ArrowDown class="size-5" />
              <p>{(totalFluctuation * 100).toLocaleString()}%</p>
            </div>
          {:else if totalFluctuation > 0}
            <div class="flex text-green-500">
              <ArrowUp class="size-5" />
              <p>{(totalFluctuation * 100).toLocaleString()}%</p>
            </div>
          {/if}
        </div>
      </div>
      <div class="flex flex-col">
        {#each portfolios as portfolio}
          {#if portfolio.asset.id === portfolioContextMenu?.id}
            <ContextMenu
              menuItems={[
                {
                  label: 'Edit',
                  icon: Pencil,
                  onClick: () => onUpdatePortfolioOpen(portfolio),
                },
                {
                  label: 'Delete',
                  icon: Trash,
                  class: 'text-red-600',
                  onClick: () => onDeleteAssetOpen(portfolio),
                },
              ]}
              position={{
                x: portfolioContextMenu.x,
                y: portfolioContextMenu.y,
              }}
            />
          {/if}
          <button
            on:contextmenu|preventDefault={(event) => {
              portfolioContextMenu = {
                id: portfolio.asset.id,
                x: event.clientX,
                y: event.clientY,
              };
            }}
          >
            <!-- Asset -->
            <div class="flex items-center justify-between border-b px-2 py-6 text-left">
              <div class="flex gap-2">
                <div class="size-16 overflow-hidden rounded-full border-2 border-white">
                  <div class="flex size-full items-center justify-center bg-gray-600 text-center">
                    <p class="text-xs text-white">{portfolio.asset.coin}</p>
                  </div>
                  <!-- <img
                   alt="EUR Logo"
                   class="size-full"
                   src="images/fiat-currencies/EUR.png"
               /> -->
                </div>
                <div class="flex flex-col justify-center">
                  <p>{portfolio.asset.coin}</p>
                  <p class="text-gray-500 dark:text-gray-400">{(1 / portfolio.asset.usd_value).toLocaleString()}</p>
                </div>
              </div>
              <div class="flex flex-col text-end">
                <p>${(portfolio.asset.usd_value * portfolio.asset.quantity).toLocaleString()}</p>
                <p class="text-gray-500 dark:text-gray-400">
                  {portfolio.asset.quantity.toLocaleString()}
                  {portfolio.asset.coin}
                </p>
              </div>
            </div>
            <!--  -->
          </button>
        {/each}
      </div>
    </div>
  </TabItem>
  {#if untaggedPortfolios.length > 0}
    <TabItem title="Untagged">
      <div class="flex flex-col">
        <div class="flex flex-col items-start gap-2 border-b px-2 py-10">
          <p class="text-gray-500 dark:text-gray-400">Total Assets</p>
          <div class="flex gap-0.5 align-top">
            <p class="text-2xl">$</p>
            <p class="text-4xl">{getTotalUSDValue(untaggedPortfolios).toLocaleString()}</p>
            {#if getTotalFluctuation(untaggedPortfolios) < 0}
              <div class="flex text-red-500">
                <ArrowDown class="size-5" />
                <p>{(getTotalFluctuation(untaggedPortfolios) * 100).toLocaleString()}%</p>
              </div>
            {:else if getTotalFluctuation(untaggedPortfolios) > 0}
              <div class="flex text-green-500">
                <ArrowUp class="size-5" />
                <p>{(getTotalFluctuation(untaggedPortfolios) * 100).toLocaleString()}%</p>
              </div>
            {/if}
          </div>
        </div>
        <div class="flex flex-col">
          {#each untaggedPortfolios as portfolio}
            <!-- Asset -->
            <div class="flex items-center justify-between border-b px-2 py-6">
              <div class="flex gap-2">
                <div class="size-16 overflow-hidden rounded-full border-2 border-white">
                  <div class="flex size-full items-center justify-center bg-gray-600 text-center">
                    <p class="text-xs text-white">{portfolio.asset.coin}</p>
                  </div>
                  <!-- <img
                   alt="EUR Logo"
                   class="size-full"
                   src="images/fiat-currencies/EUR.png"
               /> -->
                </div>
                <div class="flex flex-col justify-center">
                  <p>{portfolio.asset.coin}</p>
                  <p class="text-gray-500 dark:text-gray-400">{(1 / portfolio.asset.usd_value).toLocaleString()}</p>
                </div>
              </div>
              <div class="flex flex-col text-end">
                <p>${(portfolio.asset.usd_value * portfolio.asset.quantity).toLocaleString()}</p>
                <p class="text-gray-500 dark:text-gray-400">
                  {portfolio.asset.quantity.toLocaleString()}
                  {portfolio.asset.coin}
                </p>
              </div>
            </div>
            <!--  -->
          {/each}
        </div>
      </div>
    </TabItem>
  {/if}
  {#each groupedPortfolios.entries() as [tag, taggedPortfolios]}
    <TabItem title={tag.name}>
      <div class="flex flex-col">
        <div class="flex flex-col items-start gap-2 border-b px-2 py-10">
          <p class="text-gray-500 dark:text-gray-400">Total Assets</p>
          <div class="flex gap-0.5 align-top">
            <p class="text-2xl">$</p>
            <p class="text-4xl">{getTotalUSDValue(taggedPortfolios).toLocaleString()}</p>
            {#if getTotalFluctuation(taggedPortfolios) < 0}
              <div class="flex text-red-500">
                <ArrowDown class="size-5" />
                <p>{(getTotalFluctuation(taggedPortfolios) * 100).toLocaleString()}%</p>
              </div>
            {:else if getTotalFluctuation(taggedPortfolios) > 0}
              <div class="flex text-green-500">
                <ArrowUp class="size-5" />
                <p>{(getTotalFluctuation(taggedPortfolios) * 100).toLocaleString()}%</p>
              </div>
            {/if}
          </div>
        </div>
        <div class="flex flex-col">
          {#each taggedPortfolios as portfolio}
            <!-- Asset -->
            <div class="flex items-center justify-between border-b px-2 py-6">
              <div class="flex gap-2">
                <div class="size-16 overflow-hidden rounded-full border-2 border-white">
                  <div class="flex size-full items-center justify-center bg-gray-600 text-center">
                    <p class="text-xs text-white">{portfolio.asset.coin}</p>
                  </div>
                  <!-- <img
                   alt="EUR Logo"
                   class="size-full"
                   src="images/fiat-currencies/EUR.png"
               /> -->
                </div>
                <div class="flex flex-col justify-center">
                  <p>{portfolio.asset.coin}</p>
                  <p class="text-gray-500 dark:text-gray-400">{(1 / portfolio.asset.usd_value).toLocaleString()}</p>
                </div>
              </div>
              <div class="flex flex-col text-end">
                <p>${(portfolio.asset.usd_value * portfolio.asset.quantity).toLocaleString()}</p>
                <p class="text-gray-500 dark:text-gray-400">
                  {portfolio.asset.quantity.toLocaleString()}
                  {portfolio.asset.coin}
                </p>
              </div>
            </div>
            <!--  -->
          {/each}
        </div>
      </div>
    </TabItem>
  {/each}
</Tabs>
