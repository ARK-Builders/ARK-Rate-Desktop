<script lang="ts">
  import type { DeletePairGroupRequest } from '$lib/business/interactors/delete_pair_group/DeletePairGroupRequest';
  import type { ViewPairGroupsResponse } from '$lib/business/interactors/view_pair_groups/ViewPairGroupsResponse';
  import { Button, Modal } from 'flowbite-svelte';
  import { CircleAlert } from 'lucide-svelte';

  type PairGroup = ViewPairGroupsResponse['pair_groups'][0];

  export let pairGroup: PairGroup;
  export let onClose: () => void;
  export let onDelete: (request: DeletePairGroupRequest) => void;

  let isOpen = true;

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

<Modal
  size="xs"
  bind:open={isOpen}
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
        onDelete({
          pair_group: {
            id: pairGroup.id,
          },
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
