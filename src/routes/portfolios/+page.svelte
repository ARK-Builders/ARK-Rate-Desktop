<script lang="ts">
  import type { SaveTagRequest } from '$lib/business/interactors/save_tag/SaveTagRequest';
  import type { StorePortfoliosRequest } from '$lib/business/interactors/store_portfolios/StorePortfoliosRequest';
  import type { ViewPortfoliosResponse } from '$lib/business/interactors/view_portfolios/ViewPortfoliosResponse';
  import { toasts } from '$lib/ui/global/stores/toastStore';
  import { invoke } from '@tauri-apps/api/core';
  import { Spinner } from 'flowbite-svelte';
  import { onMount } from 'svelte';
  import EmptyView from './EmptyView.svelte';
  import FilledView from './FilledView.svelte';
  import StorePortfoliosModal from './StorePortfoliosModal.svelte';

  type Tag = ViewPortfoliosResponse['tags'][0];
  type USDPair = ViewPortfoliosResponse['usd_pairs'][0];
  type Portfolio = ViewPortfoliosResponse['portfolios'][0];

  let isLoading = false;
  let isStorePortfoliosOpen = false;

  let tags: Tag[] = [];
  let usdPairs: USDPair[] = [];
  let untaggedPortfolios: Portfolio[] = [];
  let groupedPortfolios: Map<Tag, Portfolio[]> = new Map();

  const loadPortfolios = () => {
    isLoading = true;
    tags = [];
    usdPairs = [];
    untaggedPortfolios = [];
    groupedPortfolios = new Map();
    invoke('view_portfolios')
      .then((rawResponse) => {
        const response: ViewPortfoliosResponse = JSON.parse(rawResponse as string);
        tags = response.tags;
        usdPairs = response.usd_pairs;
        response.portfolios.forEach((p) => {
          if (p.tags.length === 0) {
            untaggedPortfolios.push(p);
          } else {
            p.tags.forEach((t) => {
              let portfolios = groupedPortfolios.get(t);
              if (portfolios === undefined) {
                portfolios = [];
                groupedPortfolios.set(t, portfolios);
              }
              portfolios.push(p);
            });
          }
        });
      })
      .catch((err) => {
        console.error(err);
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'error',
            message: 'Something unexpected happened...',
          },
        ];
      })
      .finally(() => {
        isLoading = false;
      });
  };

  const onStorePortfoliosOpen = () => {
    isStorePortfoliosOpen = true;
  };

  const onStorePortfoliosClose = () => {
    isStorePortfoliosOpen = false;
  };

  const onPortfoliosStore = (request: StorePortfoliosRequest) => {
    console.log(request);
    isStorePortfoliosOpen = false;
  };

  const onTagSave = (request: SaveTagRequest) => {
    console.log(request);
  };

  onMount(() => {
    loadPortfolios();
  });
</script>

{#if isStorePortfoliosOpen}
  <StorePortfoliosModal
    {tags}
    {usdPairs}
    {onTagSave}
    onStore={onPortfoliosStore}
    onClose={onStorePortfoliosClose}
  />
{/if}

{#if isLoading}
  <div class="flex size-full items-center justify-center">
    <Spinner class="size-32" />
  </div>
{:else}
  <div class="h-full min-h-max w-full min-w-max overflow-auto p-24">
    {#if groupedPortfolios.size === 0}
      <EmptyView {onStorePortfoliosOpen} />
    {:else}
      <FilledView
        {groupedPortfolios}
        {untaggedPortfolios}
        {onStorePortfoliosOpen}
      />
    {/if}
  </div>
{/if}
