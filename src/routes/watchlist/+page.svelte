<script lang="ts">
  import type { DeleteWatchlistPairRequest } from '$lib/business/interactors/delete_watchlist_pair/DeleteWatchlistPairRequest';
  import type { ErrorResponse } from '$lib/business/interactors/ErrorResponse';
  import type { StoreWatchlistCoinsRequest } from '$lib/business/interactors/store_watchlist_coins/StoreWatchlistCoinsRequest';
  import type { ViewWatchlistResponse } from '$lib/business/interactors/view_watchlist/ViewWatchlistResponse';
  import { toasts } from '$lib/ui/global/stores/toastStore';
  import { invoke } from '@tauri-apps/api/core';
  import { Button, Heading, Spinner } from 'flowbite-svelte';
  import { Trash } from 'lucide-svelte';
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

  const onWatchlistPairDelete = async (request: DeleteWatchlistPairRequest) => {
    return invoke('delete_watchlist_pair', { request: JSON.stringify(request) })
      .then(() => {
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'success',
            message: 'Currency deleted!',
          },
        ];
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

{#if isLoading}
  <div class="flex size-full items-center justify-center">
    <Spinner class="size-32" />
  </div>
{:else}
  <div class="h-full min-w-max overflow-auto p-24">
    <!-- COIN MATRIX -->
    <div class="rounded-lg border">
      <!-- HEADER -->
      <div class="relative p-4">
        <!-- TODO -->
        <Heading tag="h5">Matrix - Multi-currency monitor</Heading>
        <!-- TODO -->
        <p>Last refreshed 2 minutes ago.</p>
        <!-- TODO -->
        <!-- <Button
        color="none"
        size="xs"
        class="absolute right-4 top-4"
      >
        <EllipsisVertical class="size-5 text-gray-500" />
      </Button> -->
      </div>
      <!-- BODY -->
      <table class="table-auto">
        <thead>
          <tr class="border-t bg-gray-100">
            <th>
              <!-- TODO -->
              <!-- <Button
              color="none"
              class="mx-auto flex w-max items-center gap-2 font-bold text-green-500"
            >
              Sort
              <ArrowRightLeft class="size-5" />
            </Button> -->
            </th>
            {#each pairs as pair}
              <th class="border-l">
                <div class="relative flex w-48 justify-center p-4">
                  <button
                    class="absolute inset-0 flex items-center justify-center bg-gray-100 text-red-500 opacity-0 hover:opacity-100"
                    on:click={() =>
                      onWatchlistPairDelete({
                        pair: {
                          id: pair.base.id,
                        },
                      })}
                  >
                    <Trash class="size-6" />
                  </button>
                  <CoinView coin={pair.base.comparison} />
                </div>
              </th>
            {/each}
            <th class="w-full border-l">
              <Button
                color="none"
                class="flex w-max items-center gap-2 pr-0 font-bold text-green-500"
                on:click={onStoreWatchlistCoinsOpen}
              >
                <span class="flex size-5 items-center justify-center rounded-full bg-green-500 p-0.5 text-white">+</span
                >
                Add currency
              </Button>
            </th>
          </tr>
        </thead>
        <tbody>
          {#each pairs as pair}
            <tr class="border-t">
              <td class="bg-gray-100">
                <div class="relative w-48 py-4 pl-6 pr-2">
                  <button
                    class="absolute inset-0 flex items-center justify-center bg-gray-100 text-red-500 opacity-0 hover:opacity-100"
                    on:click={() =>
                      onWatchlistPairDelete({
                        pair: {
                          id: pair.base.id,
                        },
                      })}
                  >
                    <Trash class="size-6" />
                  </button>
                  <CoinView coin={pair.base.comparison} />
                </div>
              </td>
              {#each pair.combinations as combination}
                <td class="border-l px-8 text-center text-sm">
                  {#if combination.comparison === pair.base.comparison}
                    1
                  {:else}
                    {combination.value.toLocaleString(undefined, {
                      currency: 'USD',
                    })}
                    {combination.comparison}
                  {/if}
                </td>
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
                <span class="flex size-5 items-center justify-center rounded-full bg-green-500 p-0.5 text-white">+</span
                >
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
{/if}
