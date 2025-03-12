<script lang="ts">
  import type { DeletePairGroupRequest } from '$lib/business/interactors/delete_pair_group/DeletePairGroupRequest';
  import type { ViewPairGroupsResponse } from '$lib/business/interactors/view_pair_groups/ViewPairGroupsResponse';
  import { Button, Modal, Spinner } from 'flowbite-svelte';
  import { CircleAlert } from 'lucide-svelte';

  type PairGroup = ViewPairGroupsResponse['pair_groups'][0];

  export let pairGroup: PairGroup;

  export let onClose: () => void;
  export let onDelete: (request: DeletePairGroupRequest) => Promise<void>;

  let isLoading = false;

  const getHeading = (): string => {
    let comparisonHeading;
    if (pairGroup.pairs.length === 1) {
      comparisonHeading = pairGroup.pairs[0].comparison;
    } else if (pairGroup.pairs.length > 3) {
      comparisonHeading = `${pairGroup.pairs
        .slice(0, 3)
        .map((p) => p.comparison)
        .join(', ')} and ${pairGroup.pairs.length - 3}+`;
    } else {
      comparisonHeading = `${pairGroup.pairs
        .slice(0, -1)
        .map((p) => p.comparison)
        .join(', ')} and ${pairGroup.pairs.at(-1)?.comparison}`;
    }
    return `${pairGroup.pairs[0].base} to ${comparisonHeading}`;
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
        Are you sure you want to delete this pair group of {getHeading()}?
      </h3>
      <Button
        color="red"
        class="me-2"
        on:click={() => {
          isLoading = true;
          onDelete({
            pair_group: {
              id: pairGroup.id,
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
