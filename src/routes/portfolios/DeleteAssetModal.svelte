<script lang="ts">
  import type { DeleteAssetRequest } from '$lib/business/interactors/delete_asset/DeleteAssetRequest';
  import type { ViewPortfoliosResponse } from '$lib/business/interactors/view_portfolios/ViewPortfoliosResponse';
  import { Button, Modal, Spinner } from 'flowbite-svelte';
  import { CircleAlert } from 'lucide-svelte';

  type Asset = ViewPortfoliosResponse['portfolios'][0]['asset'];

  export let asset: Asset;

  export let onClose: () => void;
  export let onDelete: (request: DeleteAssetRequest) => Promise<void>;

  let isLoading = false;

  const getHeading = (): string => {
    return `${asset.quantity.toLocaleString()} ${asset.coin}`;
  };
</script>

{#if isLoading}
  <Modal
    dismissable={false}
    open
    size="xs"
    classDialog="absolute max-h-screen"
    on:close={onClose}
  >
    <div class="flex size-full items-center justify-center">
      <Spinner class="size-16" />
    </div>
  </Modal>
{:else}
  <Modal
    open
    size="xs"
    classDialog="absolute max-h-screen"
    on:close={onClose}
  >
    <div class="text-center">
      <CircleAlert class="mx-auto mb-4 h-12 w-12 text-gray-400 dark:text-gray-200" />
      <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
        Are you sure you want to delete this asset of {getHeading()}?
      </h3>
      <Button
        color="red"
        class="me-2"
        on:click={() => {
          isLoading = true;
          onDelete({
            asset: {
              id: asset.id,
            },
          }).finally(() => {
            isLoading = false;
          });
        }}
      >
        Yes, I'm sure
      </Button>
      <Button
        color="alternative"
        on:click={onClose}
      >
        No, cancel
      </Button>
    </div>
  </Modal>
{/if}
