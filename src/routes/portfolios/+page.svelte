<script lang="ts">
  import type { ViewPortfoliosResponse } from '$lib/business/interactors/view_portfolios/ViewPortfoliosResponse';
  import { invoke } from '@tauri-apps/api/core';
  import { Spinner } from 'flowbite-svelte';
  import { onMount } from 'svelte';
  import { toasts } from '../layoutStore';
  import EmptyView from './EmptyView.svelte';

  type Tag = ViewPortfoliosResponse['portfolios'][0]['tag'];
  type USDPair = ViewPortfoliosResponse['usd_pairs'][0];
  type Portfolio = ViewPortfoliosResponse['portfolios'][0];

  let isLoading = false;
  let usdPairs: USDPair[] = [];
  let groupedPortfolios: Map<Tag, Portfolio[]> = new Map();

  const onSaveAssetOpen = () => {
    // TODO
  };

  const loadPortfolios = () => {
    isLoading = true;
    usdPairs = [];
    groupedPortfolios = new Map();
    invoke('view_portfolios')
      .then((rawResponse) => {
        const response: ViewPortfoliosResponse = JSON.parse(rawResponse as string);
        for (const pair of response['usd_pairs']) {
          usdPairs.push(pair);
        }
        for (const portfolio of response['portfolios']) {
          let tag = groupedPortfolios.keys().find((tag) => tag.id === portfolio.tag.id);
          if (!tag) {
            tag = portfolio['tag'];
            groupedPortfolios.set(tag, []);
          }
          const portfolios = groupedPortfolios.get(tag)!;
          portfolios.push(portfolio);
        }
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

  onMount(() => {
    loadPortfolios();
  });
</script>

{#if isLoading}
  <div class="flex size-full items-center justify-center">
    <Spinner class="size-32" />
  </div>
{:else}
  <div class="h-full min-h-max w-full min-w-max overflow-auto p-24">
    {#if true}
      <EmptyView {onSaveAssetOpen} />
    {:else}
      <!-- PASS -->
    {/if}
  </div>
{/if}
