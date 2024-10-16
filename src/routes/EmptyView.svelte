<script lang="ts">
  import { Button, Heading } from 'flowbite-svelte';
  import { Coins, Plus } from 'lucide-svelte';

  export let onCalculateClick: () => void;
</script>

<div class="flex h-full w-full items-center justify-center">
  <div class="flex max-w-sm flex-col items-center gap-6">
    <div class="flex flex-col items-center gap-2 text-center">
      <Coins class="size-16 text-green-500" />
      <Heading tag="h4">Ready for calculation!</Heading>
      <p class="text-gray-500">
        Select your currencies and enter an amount to start converting. Your exchange results will appear here.
      </p>
    </div>
    <Button
      size="lg"
      color="primary"
      class="flex gap-1"
      on:click={onCalculateClick}
    >
      <Plus />
      Calculate
    </Button>
  </div>
</div>
