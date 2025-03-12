<script lang="ts">
  import type { DeleteAssetRequest } from '$lib/business/interactors/delete_asset/DeleteAssetRequest';
  import type { ErrorResponse } from '$lib/business/interactors/ErrorResponse';
  import type { SaveTagRequest } from '$lib/business/interactors/save_tag/SaveTagRequest';
  import type { StorePortfoliosRequest } from '$lib/business/interactors/store_portfolios/StorePortfoliosRequest';
  import type { UpdatePortfolioRequest } from '$lib/business/interactors/update_portfolio/UpdatePortfolioRequest';
  import type { ViewPortfoliosResponse } from '$lib/business/interactors/view_portfolios/ViewPortfoliosResponse';
  import { toasts } from '$lib/ui/global/stores/toastStore';
  import { invoke } from '@tauri-apps/api/core';
  import { Spinner } from 'flowbite-svelte';
  import { onMount } from 'svelte';
  import DeleteAssetModal from './DeleteAssetModal.svelte';
  import EmptyView from './EmptyView.svelte';
  import FilledView from './FilledView.svelte';
  import StorePortfoliosModal from './StorePortfoliosModal.svelte';
  import UpdatePortfolioModal from './UpdatePortfolioModal.svelte';

  type Tag = ViewPortfoliosResponse['tags'][0];
  type USDPair = ViewPortfoliosResponse['usd_pairs'][0];
  type Portfolio = ViewPortfoliosResponse['portfolios'][0];

  let isLoading = false;
  let isStorePortfoliosOpen = false;

  let portfolioToUpdate: Portfolio | undefined;
  let assetToDelete: Portfolio['asset'] | undefined;

  let tags: Tag[] = [];
  let usdPairs: USDPair[] = [];
  let untaggedPortfolios: Portfolio[] = [];
  let groupedPortfolios: Map<Tag, Portfolio[]> = new Map();

  const loadPortfolios = async (): Promise<void> => {
    isLoading = true;
    tags = [];
    usdPairs = [];
    untaggedPortfolios = [];
    groupedPortfolios = new Map();
    return invoke('view_portfolios')
      .then((rawResponse) => {
        const response: ViewPortfoliosResponse = JSON.parse(rawResponse as string);
        tags = response.tags;
        usdPairs = response.usd_pairs;
        response.portfolios.forEach((p) => {
          if (p.tags.length === 0) {
            untaggedPortfolios.push(p);
          } else {
            p.tags.forEach((t) => {
              const foundKey = [...groupedPortfolios.keys()].find((k) => k.id === t.id);
              if (foundKey === undefined) {
                groupedPortfolios.set(t, [p]);
              } else {
                const portfolios = groupedPortfolios.get(foundKey)!;
                portfolios.push(p);
              }
            });
          }
        });
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

  const onStorePortfoliosOpen = () => {
    isStorePortfoliosOpen = true;
  };

  const onStorePortfoliosClose = () => {
    isStorePortfoliosOpen = false;
  };

  const onPortfoliosStore = async (request: StorePortfoliosRequest): Promise<void> => {
    return invoke('store_portfolios', { request: JSON.stringify(request) })
      .then(() => {
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'success',
            message: 'Assets saved successfully!',
          },
        ];
        isStorePortfoliosOpen = false;
        return loadPortfolios();
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

  const onTagSave = async (request: SaveTagRequest): Promise<void> => {
    return invoke('save_tag', { request: JSON.stringify(request) })
      .then(() => {
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'success',
            message: 'Tag saved successfully!',
          },
        ];
        return loadPortfolios();
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

  const onUpdatePortfolioOpen = (portfolio: Portfolio) => {
    portfolioToUpdate = structuredClone(portfolio);
  };

  const onUpdatePortfolioClose = () => {
    portfolioToUpdate = undefined;
  };

  const onPortfolioUpdate = async (request: UpdatePortfolioRequest): Promise<void> => {
    return invoke('update_portfolio', { request: JSON.stringify(request) })
      .then(() => {
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'success',
            message: 'Portfolio updated successfully!',
          },
        ];
        portfolioToUpdate = undefined;
        return loadPortfolios();
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

  const onDeleteAssetOpen = (portfolio: Portfolio) => {
    assetToDelete = structuredClone(portfolio.asset);
  };

  const onDeleteAssetClose = () => {
    assetToDelete = undefined;
  };

  const onAssetDelete = async (request: DeleteAssetRequest): Promise<void> => {
    return invoke('delete_asset', { request: JSON.stringify(request) })
      .then(() => {
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'success',
            message: 'Asset deleted!',
          },
        ];
        assetToDelete = undefined;
        return loadPortfolios();
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
{:else if portfolioToUpdate}
  <UpdatePortfolioModal
    portfolio={portfolioToUpdate}
    {tags}
    {usdPairs}
    {onTagSave}
    onUpdate={onPortfolioUpdate}
    onClose={onUpdatePortfolioClose}
  />
{:else if assetToDelete}
  <DeleteAssetModal
    asset={assetToDelete}
    onDelete={onAssetDelete}
    onClose={onDeleteAssetClose}
  />
{/if}

{#if isLoading}
  <div class="flex size-full items-center justify-center">
    <Spinner class="size-32" />
  </div>
{:else}
  <div class="h-full min-h-max w-full min-w-max overflow-auto p-24">
    {#if groupedPortfolios.size === 0 && untaggedPortfolios.length === 0}
      <EmptyView {onStorePortfoliosOpen} />
    {:else}
      <FilledView
        {groupedPortfolios}
        {untaggedPortfolios}
        {onDeleteAssetOpen}
        {onStorePortfoliosOpen}
        {onUpdatePortfolioOpen}
      />
    {/if}
  </div>
{/if}
