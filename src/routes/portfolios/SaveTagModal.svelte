<script lang="ts">
  import type { SaveTagRequest } from '$lib/business/interactors/save_tag/SaveTagRequest';
  import { Button, Heading, Input, Label, Modal, Spinner } from 'flowbite-svelte';
  import { FolderClosed } from 'lucide-svelte';

  export let onClose: () => void;
  export let onSave: (request: SaveTagRequest) => void;

  let isLoading = false;
  let isDisabled = false;

  let insertedTag: string = '';
</script>

<Modal
  open
  size="xs"
  classDialog="absolute max-h-screen"
  on:close={onClose}
>
  {#if isLoading}
    <div class="flex size-full items-center justify-center">
      <Spinner class="size-16" />
    </div>
  {:else}
    <div class="flex flex-col gap-4">
      <FolderClosed class="size-12 rounded-md border p-2" />
      <div class="flex flex-col gap-4 px-4">
        <div class="flex flex-col">
          <Heading tag="h5">Create Tag</Heading>
          <p>Please enter a name for this tag.</p>
        </div>
        <form class="flex flex-col">
          <Label class="space-y-2">
            <span>Tag name</span>
            <Input
              bind:value={insertedTag}
              size="lg"
              placeholder="e.g. Tag 1"
            />
          </Label>
        </form>
      </div>
    </div>
  {/if}

  <div
    slot="footer"
    class="flex w-full justify-end gap-4"
  >
    <Button
      color="light"
      on:click={onClose}
    >
      Cancel
    </Button>
    <Button
      disabled={isDisabled || isLoading || insertedTag.trim().length === 0}
      color="primary"
      on:click={() => {
        isLoading = true;
        onSave({
          tag: {
            name: insertedTag,
          },
        });
      }}
    >
      Save Changes
    </Button>
  </div>
</Modal>
