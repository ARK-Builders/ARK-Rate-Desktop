<script lang="ts">
  import { Button, Heading } from 'flowbite-svelte';
  import { ChartNoAxesColumn, Plus } from 'lucide-svelte';

  export let onStorePortfoliosOpen: () => void;
</script>

<div class="flex h-full w-full items-center justify-center">
  <div class="flex max-w-sm flex-col items-center gap-6">
    <div class="flex flex-col items-center gap-2 text-center">
      <ChartNoAxesColumn class="size-16 text-green-500" />
      <Heading tag="h4">Your Portfolio is Empty</Heading>
      <p class="text-gray-500">
        Add currencies manualy to track and compare. Your portfolio helps you monitor and manage your favorite or
        frequently used currencies.
      </p>
    </div>
    <Button
      size="lg"
      color="primary"
      class="flex gap-1"
      on:click={onStorePortfoliosOpen}
    >
      <Plus />
      New Asset
    </Button>
  </div>
</div>
