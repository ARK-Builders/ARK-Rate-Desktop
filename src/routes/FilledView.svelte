<script lang="ts">
  import type { ViewPairGroupsResponse } from '$lib/business/interactors/view_pair_groups/ViewPairGroupsResponse';
  import ContextMenu from '$lib/ui/global/components/context_menu/ContextMenu.svelte';
  import { Button, Heading } from 'flowbite-svelte';
  import { Coins, Pencil, Pin, PinOff, Plus, Trash } from 'lucide-svelte';
  import PairGroupView from './PairGroupView.svelte';

  type PairGroup = ViewPairGroupsResponse['pair_groups'][0];

  export let pinnedPairGroups: PairGroup[];
  export let unpinnedPairGroups: PairGroup[];

  export let onSavePairGroupOpen: () => void;
  export let onPairGroupPinToggle: (pairGroup: PairGroup) => void;
  export let onUpdatePairGroupOpen: (pairGroup: PairGroup) => void;
  export let onDeletePairGroupOpen: (pairGroup: PairGroup) => void;

  let contextMenu:
    | {
        id: string;
        x: number;
        y: number;
      }
    | undefined;
</script>

<svelte:window on:click={() => (contextMenu = undefined)} />

<div class="flex flex-col gap-16">
  <div class="flex items-center justify-between gap-12">
    <div class="flex gap-4">
      <Coins class="size-16 text-green-500" />
      <div class="flex flex-col gap-1">
        <Heading tag="h4">Ready for calculation!</Heading>
        <p class="max-w-2xl text-gray-500">
          Select your currencies and enter an amount to start converting. Your exchange results will appear here.
        </p>
      </div>
    </div>
    <Button
      size="md"
      color="primary"
      class="flex gap-1"
      on:click={onSavePairGroupOpen}
    >
      <Plus />
      Calculate
    </Button>
  </div>
  {#if pinnedPairGroups.length > 0}
    <div class="flex flex-col">
      <div class="flex items-center justify-between gap-12 border-b py-3 text-gray-500">
        <p class="font-bold">Pinned pairs</p>
        <Pin class="size-5 rotate-45 fill-gray-500" />
      </div>
      {#each pinnedPairGroups as pairGroup}
        {#if pairGroup.id === contextMenu?.id}
          <ContextMenu
            menuItems={[
              {
                label: 'Unpin',
                icon: PinOff,
                onClick: () => onPairGroupPinToggle(pairGroup),
              },
              {
                label: 'Edit',
                icon: Pencil,
                onClick: () => onUpdatePairGroupOpen(pairGroup),
              },
              {
                label: 'Delete',
                icon: Trash,
                class: 'text-red-600',
                onClick: () => onDeletePairGroupOpen(pairGroup),
              },
            ]}
            position={{
              x: contextMenu.x,
              y: contextMenu.y,
            }}
          />
        {/if}
        <button
          on:contextmenu|preventDefault={(event) => {
            contextMenu = {
              id: pairGroup.id,
              x: event.clientX,
              y: event.clientY,
            };
          }}
        >
          <PairGroupView {pairGroup} />
        </button>
      {/each}
    </div>
  {/if}
  {#if unpinnedPairGroups.length > 0}
    <div class="flex flex-col">
      <div class="flex items-center justify-between gap-12 border-b py-3 text-gray-500">
        <p class="font-bold">Calculated pairs</p>
      </div>
      {#each unpinnedPairGroups as pairGroup}
        {#if pairGroup.id === contextMenu?.id}
          <ContextMenu
            menuItems={[
              {
                label: 'Pin',
                icon: Pin,
                onClick: () => onPairGroupPinToggle(pairGroup),
              },
              {
                label: 'Edit',
                icon: Pencil,
                onClick: () => onUpdatePairGroupOpen(pairGroup),
              },
              {
                label: 'Delete',
                icon: Trash,
                class: 'text-red-600',
                onClick: () => onDeletePairGroupOpen(pairGroup),
              },
            ]}
            position={{
              x: contextMenu.x,
              y: contextMenu.y,
            }}
          />
        {/if}
        <button
          on:contextmenu|preventDefault={(event) => {
            contextMenu = {
              id: pairGroup.id,
              x: event.clientX,
              y: event.clientY,
            };
          }}
        >
          <PairGroupView {pairGroup} />
        </button>
      {/each}
    </div>
  {/if}
</div>
