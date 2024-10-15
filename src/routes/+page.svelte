<script lang="ts">
  import { Button, ButtonGroup, Heading, Input, InputAddon, Label, Modal, Select } from 'flowbite-svelte';
  import { Coins, Plus } from 'lucide-svelte';

  let isAddNewPairOpen = true;
</script>

<Modal
  bind:open={isAddNewPairOpen}
  outsideclose
  size="xs"
  classDialog="absolute"
  title="Add New Pair"
>
  <form>
    <Label class="space-y-2">
      <span> From </span>
      <ButtonGroup class="w-full">
        <InputAddon
        class="p-0"
        >
          <Select
          class="rounded-none"
            items={[
              { value: 'usd', name: 'USD' },
              { value: 'cad', name: 'CAD' },
            ]}
          />
        </InputAddon>
        <Input type="text" />
      </ButtonGroup>
    </Label>
  </form>
</Modal>

<div class="h-full min-h-max w-full min-w-max overflow-auto p-24">
  {#if true}
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
          on:click={() => (isAddNewPairOpen = true)}
        >
          <Plus />
          Calculate
        </Button>
      </div>
    </div>
  {:else}
    <!-- else content here -->
  {/if}
</div>
