<script lang="ts">
  import type { PairGroup } from '$lib/business/entities/PairGroup';
  import { Button, Heading } from 'flowbite-svelte';
  import { Coins, Pin, Plus } from 'lucide-svelte';
  import PairGroupView from './PairGroupView.svelte';

  export let pinnedPairGroups: PairGroup[];
  export let calculatedPairGroups: PairGroup[];
  export let frequentPairGroups: PairGroup[];

  export let onCalculateClick: () => void;
</script>

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
        <PairGroupView />
      {/each}
    </div>
  {/if}
  {#if calculatedPairGroups.length > 0}
    <div class="flex flex-col">
      <div class="flex items-center justify-between gap-12 border-b py-3 text-gray-500">
        <p class="font-bold">Calculated pairs</p>
      </div>
      {#each calculatedPairGroups as pairGroup}
        <PairGroupView />
      {/each}
    </div>
  {/if}
  {#if frequentPairGroups.length > 0}
    <div class="flex flex-col">
      <div class="flex items-center justify-between gap-12 border-b py-3 text-gray-500">
        <p class="font-bold">Frequent pairs</p>
        <Pin class="size-5 rotate-45 fill-gray-500" />
      </div>
      {#each frequentPairGroups as pairGroup}
        <PairGroupView />
      {/each}
    </div>
  {/if}
</div>
