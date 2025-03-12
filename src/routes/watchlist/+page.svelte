<script lang="ts">
  import type { DeleteWatchlistPairRequest } from '$lib/business/interactors/delete_watchlist_pair/DeleteWatchlistPairRequest';
  import type { ErrorResponse } from '$lib/business/interactors/ErrorResponse';
  import type { StoreWatchlistCoinsRequest } from '$lib/business/interactors/store_watchlist_coins/StoreWatchlistCoinsRequest';
  import type { ViewWatchlistResponse } from '$lib/business/interactors/view_watchlist/ViewWatchlistResponse';
  import { toasts } from '$lib/ui/global/stores/toastStore';
  import { invoke } from '@tauri-apps/api/core';
  import { Button, Heading, Spinner } from 'flowbite-svelte';
  import { ArrowDown, ArrowRightLeft, ArrowUp, Binoculars, Grip, Trash } from 'lucide-svelte';
  import { DateTime, Duration } from 'luxon';
  import { onMount } from 'svelte';
  import CoinView from './CoinView.svelte';
  import StoreWatchlistCoinsModal from './StoreWatchlistCoinsModal.svelte';

  type Pair = ViewWatchlistResponse['pairs'][0];
  type Combination = {
    row: Pair;
    column: Pair;
    value: number;
    fluctuation: number;
  };

  let isSorting = false;
  let isLoading = false;
  let isStoreWatchlistCoinsOpen = false;

  let rows: Pair[] = [];
  let columns: Pair[] = [];
  let coins: string[] = [];
  let now: DateTime = DateTime.now();
  let updatedAt: DateTime = DateTime.now();

  $: getLastUpdateMessage = (): string => {
    const duration = Duration.fromMillis(now.toMillis() - updatedAt.toMillis());
    return duration.shiftTo('minutes').toHuman({
      listStyle: 'narrow',
      unitDisplay: 'narrow',
      roundingMode: 'floor',
      maximumFractionDigits: 0,
    });
  };

  $: getCombinations = (row: Pair): Combination[] => {
    return columns.map((column) => {
      if (column.id === row.id) {
        return {
          row,
          column,
          value: 1,
          fluctuation: 0,
        };
      }
      return {
        row,
        column,
        value: column.value / row.value,
        fluctuation: column.fluctuation - row.fluctuation,
      };
    });
  };

  const loadWatchlist = async () => {
    isLoading = true;
    rows = [];
    coins = [];
    now = DateTime.now();
    updatedAt = DateTime.now();
    return invoke('view_watchlist')
      .then((rawResponse) => {
        const response: ViewWatchlistResponse = JSON.parse(rawResponse as string);
        coins = response.coins;
        rows = [...response.pairs];
        columns = [...response.pairs];
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

  const onSortToggle = () => {
    isSorting = !isSorting;
  };

  const onColumnDragStart = (column: Pair) => (event: DragEvent) => {
    if (!event.dataTransfer) return;
    event.dataTransfer.setData('text/plain', column.id);
  };

  const onColumnDrop = (column: Pair) => (event: DragEvent) => {
    if (!event.dataTransfer) return;
    const columnIndex = columns.findIndex((c) => c.id === column.id);
    if (columnIndex < 0) return;
    const replacementId = event.dataTransfer.getData('text/plain');
    const replacementIndex = columns.findIndex((c) => c.id === replacementId);
    if (replacementIndex < 0) return;
    const replacement = columns.splice(replacementIndex, 1);
    columns.splice(columnIndex, 0, ...replacement);
    columns = columns;
  };

  const onRowDragStart = (row: Pair) => (event: DragEvent) => {
    if (!event.dataTransfer) return;
    event.dataTransfer.setData('text/plain', row.id);
  };

  const onRowDrop = (row: Pair) => (event: DragEvent) => {
    if (!event.dataTransfer) return;
    const rowIndex = rows.findIndex((c) => c.id === row.id);
    if (rowIndex < 0) return;
    const replacementId = event.dataTransfer.getData('text/plain');
    const replacementIndex = rows.findIndex((c) => c.id === replacementId);
    if (replacementIndex < 0) return;
    const replacement = rows.splice(replacementIndex, 1);
    rows.splice(rowIndex, 0, ...replacement);
    rows = rows;
  };

  onMount(() => {
    loadWatchlist();
    const nowInterval = setInterval(() => {
      now = DateTime.now();
    }, 60000);
    return () => {
      clearInterval(nowInterval);
    };
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
        <div class="flex items-center gap-2">
          <Binoculars class="size-8 text-primary-500" />
          <Heading tag="h5">Watchlist - Multi-currency monitor</Heading>
        </div>
        <p>Last updated {getLastUpdateMessage()} ago</p>
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
            <th class={isSorting ? 'animate-pulse bg-green-500' : ''}>
              <!-- TODO -->
              <Button
                color="none"
                class="mx-auto flex w-max items-center gap-2 font-bold {isSorting ? 'text-white' : 'text-green-500'}"
                on:click={onSortToggle}
              >
                Sort
                <ArrowRightLeft class="size-5" />
              </Button>
            </th>
            {#each columns as column}
              <th class="border-l">
                <div class="relative flex w-48 justify-center p-4">
                  {#if isSorting}
                    <button
                      draggable="true"
                      on:dragover|preventDefault
                      on:drop={onColumnDrop(column)}
                      on:dragstart={onColumnDragStart(column)}
                      class="absolute inset-0 flex items-center justify-center bg-gray-100 opacity-0 hover:cursor-move hover:opacity-100 active:cursor-grabbing"
                    >
                      <Grip class="size-6" />
                    </button>
                  {:else}
                    <button
                      class="absolute inset-0 flex items-center justify-center bg-gray-100 text-red-500 opacity-0 hover:opacity-100"
                      on:click={() =>
                        onWatchlistPairDelete({
                          pair: {
                            id: column.id,
                          },
                        })}
                    >
                      <Trash class="size-6" />
                    </button>
                  {/if}
                  <CoinView coin={column.comparison} />
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
          {#each rows as row}
            <tr class="border-t">
              <td class="bg-gray-100">
                <div class="relative w-48 py-4 pl-6 pr-2">
                  {#if isSorting}
                    <button
                      draggable="true"
                      on:dragover|preventDefault
                      on:drop={onRowDrop(row)}
                      on:dragstart={onRowDragStart(row)}
                      class="absolute inset-0 flex items-center justify-center bg-gray-100 opacity-0 hover:cursor-move hover:opacity-100 active:cursor-grabbing"
                    >
                      <Grip class="size-6" />
                    </button>
                  {:else}
                    <button
                      class="absolute inset-0 flex items-center justify-center bg-gray-100 text-red-500 opacity-0 hover:opacity-100"
                      on:click={() =>
                        onWatchlistPairDelete({
                          pair: {
                            id: row.id,
                          },
                        })}
                    >
                      <Trash class="size-6" />
                    </button>
                  {/if}
                  <CoinView coin={row.comparison} />
                </div>
              </td>
              {#each getCombinations(row) as combination}
                <td class="border-l">
                  <div class="flex min-w-max flex-col items-center gap-2 p-4">
                    {#if combination.row.id === combination.column.id}
                      1
                    {:else}
                      {combination.value.toLocaleString()}
                      {combination.column.comparison}
                    {/if}
                    {#if combination.fluctuation > 0}
                      <div
                        class="flex items-center rounded-full border border-green-300 bg-green-100 p-1 text-xs text-green-500"
                      >
                        <ArrowUp class="size-3" />
                        %{(combination.fluctuation * 100).toLocaleString(undefined, {
                          maximumFractionDigits: 2,
                        })}
                      </div>
                    {:else if combination.fluctuation < 0}
                      <div
                        class="flex items-center rounded-full border border-red-300 bg-red-100 p-1 text-xs text-red-500"
                      >
                        <ArrowDown class="size-3" />
                        %{(combination.fluctuation * 100).toLocaleString(undefined, {
                          maximumFractionDigits: 2,
                        })}
                      </div>
                    {/if}
                  </div>
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
            {#each rows as _}
              <td class="border-l"></td>
            {/each}
            <td class="border-l"></td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
{/if}
