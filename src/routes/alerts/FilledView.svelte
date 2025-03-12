<script lang="ts">
  import { Button, Heading } from 'flowbite-svelte';
  import { Bell, Plus } from 'lucide-svelte';

  export let onAddAlertOpen: () => void;
</script>

<div class="mb-16 flex items-center justify-between gap-12">
  <div class="flex gap-4">
    <Bell class="size-16 text-green-500" />
    <div class="flex flex-col gap-1">
      <Heading tag="h4">Stay Updated</Heading>
      <p class="max-w-2xl text-wrap text-gray-500">
        We'll post any important notifications or changes in exchange rates here.
      </p>
    </div>
  </div>
  <Button
    size="md"
    color="primary"
    class="flex gap-1"
    on:click={onAddAlertOpen}
  >
    <Plus />
    New Alert
  </Button>
</div>
