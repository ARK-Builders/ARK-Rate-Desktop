<script lang="ts">
  import type { PairGroup } from '$lib/business/entities/PairGroup';
  import { Button, Heading } from 'flowbite-svelte';
  import { Coins, Pencil, Pin, PinOff, Plus, Trash } from 'lucide-svelte';
  import ContextMenu from './ContextMenu.svelte';
  import PairGroupView from './PairGroupView.svelte';

  export let pinnedPairGroups: PairGroup[];
  export let unpinnedPairGroups: PairGroup[];
  export let onCalculateClick: () => void;
  export let onPairGroupUpdateClick: (pairGroup: PairGroup) => void;
  export let onPairGroupDeleteClick: (pairGroup: PairGroup) => void;
  export let onPairGroupPinToggleClick: (pairGroup: PairGroup) => void;

  let openContextMenu:
    | {
        id: string;
        x: number;
        y: number;
      }
    | undefined;
</script>

<svelte:window on:click={() => (openContextMenu = undefined)} />

<div class="flex flex-col gap-16">
  <div class="flex items-center justify-between gap-12">
    <div class="flex gap-4">
      <Coins class="size-16 text-green-500" />
      <div class="flex flex-col gap-1">
        <Heading tag="h4">Ready for calculation!</Heading>
        <p class="text-gray-500">
          Select your currencies and enter an amount to start converting. Your exchange results will appear here.
        </p>
      </div>
    </div>
    <Button
      size="md"
      color="primary"
      class="flex gap-1"
      on:click={onCalculateClick}
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
        {#if pairGroup.id === openContextMenu?.id}
          <ContextMenu
            menuItems={[
              {
                label: 'Unpin',
                icon: PinOff,
                onClick: () => onPairGroupPinToggleClick(pairGroup),
              },
              {
                label: 'Edit',
                icon: Pencil,
                onClick: () => onPairGroupUpdateClick(pairGroup),
              },
              {
                label: 'Delete',
                icon: Trash,
                class: 'text-red-600',
                onClick: () => onPairGroupDeleteClick(pairGroup),
              },
            ]}
            position={{
              x: openContextMenu.x,
              y: openContextMenu.y,
            }}
          />
        {/if}
        <button
          on:contextmenu|preventDefault={(event) => {
            openContextMenu = {
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
        {#if pairGroup.id === openContextMenu?.id}
          <ContextMenu
            menuItems={[
              {
                label: 'Pin',
                icon: Pin,
                onClick: () => onPairGroupPinToggleClick(pairGroup),
              },
              {
                label: 'Edit',
                icon: Pencil,
                onClick: () => onPairGroupUpdateClick(pairGroup),
              },
              {
                label: 'Delete',
                icon: Trash,
                class: 'text-red-600',
                onClick: () => onPairGroupDeleteClick(pairGroup),
              },
            ]}
            position={{
              x: openContextMenu.x,
              y: openContextMenu.y,
            }}
          />
        {/if}
        <button
          on:contextmenu|preventDefault={(event) => {
            openContextMenu = {
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
