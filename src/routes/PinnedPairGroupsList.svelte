<script lang="ts">
  import type { ViewPairGroupsResponse } from '$lib/business/interactors/view_pair_groups/ViewPairGroupsResponse';
  import ContextMenu from '$lib/ui/global/components/context_menu/ContextMenu.svelte';
  import { Pencil, PinOff, Trash } from 'lucide-svelte';
  import PairGroupView from './PairGroupView.svelte';

  type PairGroup = ViewPairGroupsResponse['pair_groups'][0];

  export let pairGroups: PairGroup[];

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

{#each pairGroups as pairGroup}
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
