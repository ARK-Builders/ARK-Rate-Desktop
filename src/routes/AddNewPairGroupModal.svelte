<script lang="ts">
  import { Button, Hr, Input, Label, Modal } from 'flowbite-svelte';
  import { ArrowDownUp, Plus, Trash } from 'lucide-svelte';
  import MultiSelect from 'svelte-multiselect';

  export let isOpen: boolean;
</script>

<Modal
  bind:open={isOpen}
  size="xs"
  title="Add New Pair"
  classDialog="absolute"
>
  <form class="flex flex-col">
    <Label class="space-y-2">
      <span>From</span>
      <div class="flex">
        <MultiSelect
          options={[
            { value: 'BRL', label: 'BRL' },
            { value: 'USD', label: 'USD' },
            { value: 'CAD', label: 'CAD' },
          ]}
          maxSelect={1}
          minSelect={1}
          liSelectedClass="!bg-transparent"
          ulOptionsClass="!rounded-tr-none"
          ulSelectedClass="!flex-grow !w-full"
          outerDivClass="!rounded-r-none !w-24"
        />
        <Input
          type="text"
          class="rounded-l-none"
        />
      </div>
    </Label>
    <div class="relative">
      <Hr />
      <div class="absolute top-0 flex h-full w-full items-center justify-center">
        <div class="rounded-full bg-white p-3">
          <div class="rounded-full border p-2">
            <ArrowDownUp class="size-4" />
          </div>
        </div>
      </div>
    </div>
    <Label class="space-y-2">
      <span>To</span>
      <div class="flex flex-col gap-4">
        <div class="flex items-center gap-4">
          <div class="flex flex-grow">
            <MultiSelect
              options={[
                { value: 'BRL', label: 'BRL' },
                { value: 'USD', label: 'USD' },
                { value: 'CAD', label: 'CAD' },
              ]}
              maxSelect={1}
              minSelect={1}
              liSelectedClass="!bg-transparent"
              ulOptionsClass="!rounded-tr-none"
              ulSelectedClass="!flex-grow !w-full"
              outerDivClass="!rounded-r-none !w-24"
            />
            <Input
              type="text"
              class="rounded-l-none"
            />
          </div>
          <Button
            outline
            size="xs"
            color="light"
          >
            <Trash class="size-4" />
          </Button>
        </div>
        <div class="flex items-center gap-4">
          <div class="flex flex-grow">
            <MultiSelect
              options={[
                { value: 'BRL', label: 'BRL' },
                { value: 'USD', label: 'USD' },
                { value: 'CAD', label: 'CAD' },
              ]}
              maxSelect={1}
              minSelect={1}
              liSelectedClass="!bg-transparent"
              ulOptionsClass="!rounded-tr-none"
              ulSelectedClass="!flex-grow !w-full"
              outerDivClass="!rounded-r-none !w-24"
            />
            <Input
              type="text"
              class="rounded-l-none"
            />
          </div>
          <Button
            outline
            size="xs"
            color="light"
          >
            <Trash class="size-4" />
          </Button>
        </div>
        <div class="flex justify-start">
          <Button
            size="xs"
            color="light"
            class="flex gap-2"
          >
            <Plus class="size-4" />
            New Currency
          </Button>
        </div>
      </div>
    </Label>
  </form>
  <div
    slot="footer"
    class="flex w-full justify-end gap-4"
  >
    <Button color="light">Cancel</Button>
    <Button color="primary">Save Changes</Button>
  </div>
</Modal>
