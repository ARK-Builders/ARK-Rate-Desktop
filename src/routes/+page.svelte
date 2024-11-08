<script lang="ts">
  import type { DeletePairGroupRequest } from '$lib/business/interactors/delete_pair_group/DeletePairGroupRequest';
  import type { SavePairGroupRequest } from '$lib/business/interactors/save_pair_group/SavePairGroupRequest';
  import type { UpdatePairGroupRequest } from '$lib/business/interactors/update_pair_group/UpdatePairGroupRequest';
  import type { ViewPairGroupsResponse } from '$lib/business/interactors/view_pair_groups/ViewPairGroupsResponse';
  import { toasts } from '$lib/ui/global/stores/toastStore';
  import { invoke } from '@tauri-apps/api/core';
  import { Spinner } from 'flowbite-svelte';
  import { onMount } from 'svelte';
  import DeletePairGroupModal from './DeletePairGroupModal.svelte';
  import EmptyView from './EmptyView.svelte';
  import FilledView from './FilledView.svelte';
  import SavePairGroupModal from './SavePairGroupModal.svelte';
  import UpdatePairGroupModal from './UpdatePairGroupModal.svelte';

  type USDPair = ViewPairGroupsResponse['usd_pairs'][0];
  type PairGroup = ViewPairGroupsResponse['pair_groups'][0];

  let isLoading = false;
  let isSavePairGroupOpen = false;

  let pairGroupToUpdate: PairGroup | undefined;
  let pairGroupToDelete: PairGroup | undefined;

  let usdPairs: USDPair[] = [];
  let pinnedPairGroups: PairGroup[] = [];
  let unpinnedPairGroups: PairGroup[] = [];

  const loadPairGroups = () => {
    isLoading = true;
    usdPairs = [];
    pinnedPairGroups = [];
    unpinnedPairGroups = [];
    invoke('view_pair_groups')
      .then((rawResponse) => {
        const response: ViewPairGroupsResponse = JSON.parse(rawResponse as string);
        for (const pair of response['usd_pairs']) {
          usdPairs.push(pair);
        }
        for (const pairGroup of response['pair_groups']) {
          if (pairGroup['is_pinned']) {
            pinnedPairGroups.push(pairGroup);
          } else {
            unpinnedPairGroups.push(pairGroup);
          }
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

  const onSavePairGroupOpen = () => {
    isSavePairGroupOpen = true;
  };

  const onSavePairGroupClose = () => {
    isSavePairGroupOpen = false;
  };

  const onPairGroupSave = (request: SavePairGroupRequest) => {
    isLoading = true;
    invoke('save_pair_group', { request: JSON.stringify(request) })
      .then(() => {
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'success',
            message: 'Pair saved successfully!',
          },
        ];
      })
      .catch((err) => {
        console.error(err);
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'error',
            message: 'Unexpected error saving pair...',
          },
        ];
      })
      .finally(() => {
        loadPairGroups();
        isSavePairGroupOpen = false;
      });
  };

  const onPairGroupPinToggle = (pairGroup: PairGroup) => {
    isLoading = true;
    invoke('update_pair_group', {
      request: JSON.stringify({
        pair_group: {
          id: pairGroup.id,
          is_pinned: !pairGroup.is_pinned,
          multiplier: pairGroup.multiplier,
          pairs: pairGroup.pairs.map((p) => ({
            id: p.id,
            base: p.base,
            value: p.value,
            comparison: p.comparison,
          })),
        },
      } as UpdatePairGroupRequest),
    })
      .then(() => {
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'success',
            message: 'Pair pin successfully updated!',
          },
        ];
      })
      .catch((err) => {
        console.error(err);
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'error',
            message: 'Unexpected error updating pair pin...',
          },
        ];
      })
      .finally(() => {
        loadPairGroups();
      });
  };

  const onUpdatePairGroupOpen = (pairGroup: PairGroup) => {
    pairGroupToUpdate = structuredClone(pairGroup);
  };

  const onUpdatePairGroupClose = () => {
    pairGroupToUpdate = undefined;
  };

  const onPairGroupUpdate = (request: UpdatePairGroupRequest) => {
    isLoading = true;
    invoke('update_pair_group', { request: JSON.stringify(request) })
      .then(() => {
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'success',
            message: 'Pair updated successfully!',
          },
        ];
      })
      .catch((err) => {
        console.error(err);
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'error',
            message: 'Unexpected error updating pair group...',
          },
        ];
      })
      .finally(() => {
        loadPairGroups();
        pairGroupToUpdate = undefined;
      });
  };

  const onDeletePairGroupOpen = (pairGroup: PairGroup) => {
    pairGroupToDelete = structuredClone(pairGroup);
  };

  const onDeletePairGroupClose = () => {
    pairGroupToDelete = undefined;
  };

  const onPairGroupDelete = (request: DeletePairGroupRequest) => {
    isLoading = true;
    invoke('delete_pair_group', { request: JSON.stringify(request) })
      .then(() => {
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'success',
            message: 'Pair updated successfully!',
          },
        ];
      })
      .catch((err) => {
        console.error(err);
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'error',
            message: 'Unexpected error updating pair group...',
          },
        ];
      })
      .finally(() => {
        loadPairGroups();
        pairGroupToDelete = undefined;
      });
  };

  onMount(() => {
    loadPairGroups();
  });
</script>

{#if isSavePairGroupOpen}
  <SavePairGroupModal
    {usdPairs}
    onSave={onPairGroupSave}
    onClose={onSavePairGroupClose}
  />
{:else if pairGroupToUpdate}
  <UpdatePairGroupModal
    {usdPairs}
    pairGroup={pairGroupToUpdate}
    onUpdate={onPairGroupUpdate}
    onClose={onUpdatePairGroupClose}
  />
{:else if pairGroupToDelete}
  <DeletePairGroupModal
    pairGroup={pairGroupToDelete}
    onDelete={onPairGroupDelete}
    onClose={onDeletePairGroupClose}
  />
{/if}

{#if isLoading}
  <div class="flex size-full items-center justify-center">
    <Spinner class="size-32" />
  </div>
{:else}
  <div class="h-full min-h-max w-full min-w-max overflow-auto p-24">
    {#if pinnedPairGroups.length === 0 && unpinnedPairGroups.length === 0}
      <EmptyView {onSavePairGroupOpen} />
    {:else}
      <FilledView
        {pinnedPairGroups}
        {unpinnedPairGroups}
        {onPairGroupPinToggle}
        {onSavePairGroupOpen}
        {onUpdatePairGroupOpen}
        {onDeletePairGroupOpen}
      />
    {/if}
  </div>
{/if}
