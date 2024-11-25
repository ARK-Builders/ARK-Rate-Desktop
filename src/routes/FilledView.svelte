<script lang="ts">
  import type { ViewPairGroupsResponse } from '$lib/business/interactors/view_pair_groups/ViewPairGroupsResponse';
  import { Button, Heading } from 'flowbite-svelte';
  import { Coins, Eye, Pin, Plus } from 'lucide-svelte';
  import PinnedPairGroupsList from './PinnedPairGroupsList.svelte';
  import UnpinnedPairGroupsList from './UnpinnedPairGroupsList.svelte';

  type PairGroup = ViewPairGroupsResponse['pair_groups'][0];

  export let pinnedPairGroups: PairGroup[];
  export let unpinnedPairGroups: PairGroup[];

  export let onSavePairGroupOpen: () => void;
  export let onPairGroupPinToggle: (pairGroup: PairGroup) => void;
  export let onUpdatePairGroupOpen: (pairGroup: PairGroup) => void;
  export let onDeletePairGroupOpen: (pairGroup: PairGroup) => void;

  let isPinnedPairGroupsMatrix = false;

  const togglePinnedPairGroupsMatrix = () => {
    isPinnedPairGroupsMatrix = !isPinnedPairGroupsMatrix;
  };
</script>

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
        <div class="flex items-center gap-1">
          <p class="font-bold">Pinned pairs</p>
          <Button
            size="xs"
            color="none"
            on:click={togglePinnedPairGroupsMatrix}
          >
            <Eye class="size-5" />
          </Button>
        </div>
        <Pin class="size-5 rotate-45 fill-gray-500" />
      </div>
      {#if isPinnedPairGroupsMatrix}
        <!-- content here -->
      {:else}
        <PinnedPairGroupsList
          pairGroups={pinnedPairGroups}
          {onPairGroupPinToggle}
          {onDeletePairGroupOpen}
          {onUpdatePairGroupOpen}
        />
      {/if}
    </div>
  {/if}
  <!-- UNPINNED PAIR GROUPS -->
  {#if unpinnedPairGroups.length > 0}
    <div class="flex flex-col">
      <div class="flex items-center justify-between gap-12 border-b py-3 text-gray-500">
        <p class="font-bold">Calculated pairs</p>
      </div>
      <UnpinnedPairGroupsList
        pairGroups={unpinnedPairGroups}
        {onPairGroupPinToggle}
        {onDeletePairGroupOpen}
        {onUpdatePairGroupOpen}
      />
    </div>
  {/if}
</div>
