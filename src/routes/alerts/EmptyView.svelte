<script lang="ts">
  import { Button, Heading } from 'flowbite-svelte';
  import { Bell, Plus } from 'lucide-svelte';

  export let onAddAlertOpen: () => void;
</script>

<div class="flex h-full w-full items-center justify-center">
  <div class="flex max-w-sm flex-col items-center gap-6">
    <div class="flex flex-col items-center gap-2 text-center">
      <Bell class="size-16 text-green-500" />
      <Heading tag="h4">No Alerts at the Moment</Heading>
      <p class="text-gray-500">
        Stay updated! We'll post any important notifications or changes in exchange rates here.
      </p>
    </div>
    <Button
      size="lg"
      color="primary"
      class="flex gap-1"
      on:click={onAddAlertOpen}
    >
      <Plus />
      New Alert
    </Button>
  </div>
</div>
