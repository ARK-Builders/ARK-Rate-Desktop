<script lang="ts">
  import type { SaveTagRequest } from '$lib/business/interactors/save_tag/SaveTagRequest';
  import { Button, Heading, Input, Label, Modal } from 'flowbite-svelte';
  import { FolderClosed } from 'lucide-svelte';

  export let onClose: () => void;
  export let onSave: (request: SaveTagRequest) => void;

  let isOpen = true;
  let isDisabled = false;

  let insertedTag: string = '';
</script>

<Modal
  bind:open={isOpen}
  size="xs"
  classDialog="absolute max-h-screen"
  on:close={onClose}
>
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

  <div
    slot="footer"
    class="flex w-full justify-end gap-4"
  >
    <Button
      color="light"
      on:click={() => (isOpen = false)}
    >
      Cancel
    </Button>
    <Button
      disabled={isDisabled || insertedTag.trim().length === 0}
      color="primary"
      on:click={() => {
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
