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

  const loadPairGroups = async (): Promise<void> => {
    isLoading = true;
    usdPairs = [];
    pinnedPairGroups = [];
    unpinnedPairGroups = [];
    return invoke('view_pair_groups')
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

  const onPairGroupSave = async (request: SavePairGroupRequest): Promise<void> => {
    return invoke('save_pair_group', { request: JSON.stringify(request) })
      .then(() => {
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'success',
            message: 'Pair saved successfully!',
          },
        ];
        isSavePairGroupOpen = false;
        return loadPairGroups();
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
      });
  };

  const onPairGroupPinToggle = async (pairGroup: PairGroup): Promise<void> => {
    return invoke('update_pair_group', {
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
        return loadPairGroups();
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
      });
  };

  const onUpdatePairGroupOpen = (pairGroup: PairGroup) => {
    pairGroupToUpdate = structuredClone(pairGroup);
  };

  const onUpdatePairGroupClose = () => {
    pairGroupToUpdate = undefined;
  };

  const onPairGroupUpdate = async (request: UpdatePairGroupRequest): Promise<void> => {
    return invoke('update_pair_group', { request: JSON.stringify(request) })
      .then(() => {
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'success',
            message: 'Pair updated successfully!',
          },
        ];
        pairGroupToUpdate = undefined;
        return loadPairGroups();
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
      });
  };

  const onDeletePairGroupOpen = (pairGroup: PairGroup) => {
    pairGroupToDelete = structuredClone(pairGroup);
  };

  const onDeletePairGroupClose = () => {
    pairGroupToDelete = undefined;
  };

  const onPairGroupDelete = async (request: DeletePairGroupRequest): Promise<void> => {
    return invoke('delete_pair_group', { request: JSON.stringify(request) })
      .then(() => {
        $toasts = [
          ...$toasts,
          {
            id: crypto.randomUUID(),
            type: 'success',
            message: 'Pair updated successfully!',
          },
        ];
        pairGroupToDelete = undefined;
        return loadPairGroups();
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
