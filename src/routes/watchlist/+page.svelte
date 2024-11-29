<script lang="ts">
  import type { ErrorResponse } from '$lib/business/interactors/ErrorResponse';
  import type { StoreWatchlistCoinsRequest } from '$lib/business/interactors/store_watchlist_coins/StoreWatchlistCoinsRequest';
  import type { ViewWatchlistResponse } from '$lib/business/interactors/view_watchlist/ViewWatchlistResponse';
  import { toasts } from '$lib/ui/global/stores/toastStore';
  import { invoke } from '@tauri-apps/api/core';
  import { Button, Heading } from 'flowbite-svelte';
  import { ArrowRightLeft, EllipsisVertical } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import CoinView from './CoinView.svelte';
  import StoreWatchlistCoinsModal from './StoreWatchlistCoinsModal.svelte';

  type Pair = ViewWatchlistResponse['pairs'][0];

  let isLoading = false;
  let isStoreWatchlistCoinsOpen = false;

  let pairs: Pair[] = [];
  let coins: string[] = [];

  const loadWatchlist = async () => {
    isLoading = true;
    pairs = [];
    coins = [];
    return invoke('view_watchlist')
      .then((rawResponse) => {
        const response: ViewWatchlistResponse = JSON.parse(rawResponse as string);
        coins = response.coins;
        pairs = response.pairs;
      })
      .catch((err) => {
        const response: ErrorResponse = JSON.parse(err);
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'error',
            message: response.message,
          },
        ];
      })
      .finally(() => {
        isLoading = false;
      });
  };

  const onStoreWatchlistCoinsOpen = () => {
    isStoreWatchlistCoinsOpen = true;
  };

  const onStoreWatchlistCoinsClose = () => {
    isStoreWatchlistCoinsOpen = false;
  };

  const onWatchlistCoinsStore = async (request: StoreWatchlistCoinsRequest) => {
    return invoke('store_watchlist_coins', { request: JSON.stringify(request) })
      .then(() => {
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'success',
            message: 'Currencies added successfully!',
          },
        ];
        isStoreWatchlistCoinsOpen = false;
        return loadWatchlist();
      })
      .catch((err) => {
        const response: ErrorResponse = JSON.parse(err);
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'error',
            message: response.message,
          },
        ];
      });
  };

  onMount(() => {
    loadWatchlist();
  });
</script>

{#if isStoreWatchlistCoinsOpen}
  <StoreWatchlistCoinsModal
    {coins}
    onStore={onWatchlistCoinsStore}
    onClose={onStoreWatchlistCoinsClose}
  />
{/if}

<div class="h-full min-w-max overflow-auto p-24">
  <!-- COIN MATRIX -->
  <div class="rounded-lg border">
    <!-- HEADER -->
    <div class="relative p-4">
      <!-- TODO -->
      <Heading tag="h5">Matrix - Multi-currency monitor</Heading>
      <!-- TODO -->
      <p>Last refreshed 2 minutes ago.</p>
      <Button
        color="none"
        size="xs"
        class="absolute right-4 top-4"
      >
        <EllipsisVertical class="size-5 text-gray-500" />
      </Button>
    </div>
    <!-- BODY -->
    <table class="table-auto">
      <thead>
        <tr class="border-t bg-gray-100">
          <th>
            <Button
              color="none"
              class="mx-auto flex w-max items-center gap-2 font-bold text-green-500"
            >
              <!-- TODO -->
              Sort
              <ArrowRightLeft class="size-5" />
            </Button>
          </th>
          {#each pairs as pair}
            <th class="border-l">
              <CoinView coin={pair.base.comparison} />
            </th>
          {/each}
          <th class="w-full border-l">
            <Button
              color="none"
              class="flex w-max items-center gap-2 pr-0 font-bold text-green-500"
              on:click={onStoreWatchlistCoinsOpen}
            >
              <span class="flex size-5 items-center justify-center rounded-full bg-green-500 p-0.5 text-white">+</span>
              Add currency
            </Button>
          </th>
        </tr>
      </thead>
      <tbody>
        {#each pairs as pair}
          <tr class="border-t">
            <td class="bg-gray-100">
              <CoinView coin={pair.base.comparison} />
            </td>
            {#each pair.combinations as combination}
              <td class="border-l px-8 text-center text-sm">{combination.value}</td>
            {/each}
            <td class="border-l"></td>
          </tr>
        {/each}
        <tr class="border-t">
          <td class="bg-gray-100 py-4">
            <Button
              color="none"
              class="flex w-max items-center gap-2 font-bold text-green-500"
              on:click={onStoreWatchlistCoinsOpen}
            >
              <span class="flex size-5 items-center justify-center rounded-full bg-green-500 p-0.5 text-white">+</span>
              Add currency
            </Button>
          </td>
          {#each pairs as _}
            <td class="border-l"></td>
          {/each}
          <td class="border-l"></td>
        </tr>
      </tbody>
    </table>
  </div>
</div>
