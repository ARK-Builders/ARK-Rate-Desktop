<script lang="ts">
  import type { SaveTagRequest } from '$lib/business/interactors/save_tag/SaveTagRequest';
  import type { UpdatePortfolioRequest } from '$lib/business/interactors/update_portfolio/UpdatePortfolioRequest';
  import type { ViewPortfoliosResponse } from '$lib/business/interactors/view_portfolios/ViewPortfoliosResponse';
  import { Button, Input, Modal, Spinner } from 'flowbite-svelte';
  import { FolderClosed, FolderPlus } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { MultiSelect, type ObjectOption } from 'svelte-multiselect';
  import SaveTagModal from './SaveTagModal.svelte';

  type Tag = ViewPortfoliosResponse['tags'][0];
  type USDPair = ViewPortfoliosResponse['usd_pairs'][0];
  type Portfolio = ViewPortfoliosResponse['portfolios'][0];
  type InsertedTag = {
    id: string;
    name: string;
  };
  type InsertedAsset = {
    id: string;
    coin: string;
    quantity: string;
  };

  export let tags: Tag[];
  export let usdPairs: USDPair[];
  export let portfolio: Portfolio;

  export let onClose: () => void;
  export let onTagSave: (request: SaveTagRequest) => Promise<void>;
  export let onUpdate: (request: UpdatePortfolioRequest) => Promise<void>;

  let isLoading = false;
  let isSaveTagOpen = false;
  let isUpdateDisabled = false;

  let coinOptions: string[] = [];
  let tagOptions: ObjectOption[] = [];

  let insertedTag: InsertedTag | undefined;
  let insertedAsset: InsertedAsset = {
    id: '',
    coin: '',
    quantity: '',
  };

  const onInsertedTagSelect = (event: CustomEvent) => {
    const detail = event.detail;
    if (!detail) return;
    if (detail.type === 'remove') {
      insertedTag = undefined;
      return;
    }
    const tagOption = detail.option as ObjectOption;
    const tag = tagOption.value as string;
    if (tag === 'new_tag') {
      insertedTag = undefined;
      onSaveTagOpen();
    } else {
      insertedTag = {
        id: tagOption.value as string,
        name: tagOption.label as string,
      };
    }
  };

  const onInsertedAssetCoinChange = (insertedAsset: InsertedAsset) => (event: CustomEvent) => {
    const detail = event.detail;
    if (!detail) return;
    if (detail.type === 'remove') {
      isUpdateDisabled = true;
      insertedAsset.coin = '';
      return;
    }
    isUpdateDisabled = false;
    insertedAsset.coin = detail.option;
  };

  const onSaveTagOpen = () => {
    isSaveTagOpen = true;
  };

  const onSaveTagClose = () => {
    isSaveTagOpen = false;
  };

  onMount(() => {
    coinOptions = usdPairs.map((p) => p.comparison);
    insertedTag =
      portfolio.tags.length > 0
        ? {
            id: portfolio.tags[0].id,
            name: portfolio.tags[0].name,
          }
        : undefined;
    insertedAsset = {
      id: portfolio.asset.id,
      coin: portfolio.asset.coin,
      quantity: portfolio.asset.quantity.toString(),
    };
  });

  $: tagOptions = [
    {
      value: 'new_tag',
      label: 'New tag',
    },
    ...tags.map((t) => ({
      value: t.id,
      label: t.name,
    })),
  ];
</script>

{#if isSaveTagOpen}
  <SaveTagModal
    onSave={async (request) =>
      onTagSave(request).finally(() => {
        isSaveTagOpen = false;
      })}
    onClose={onSaveTagClose}
  />
{/if}

{#if isLoading}
  <Modal
    dismissable={false}
    open
    size="xs"
    title="Add New Assets"
    classDialog="absolute max-h-screen"
    on:close={onClose}
  >
    <div class="flex size-full items-center justify-center">
      <Spinner class="size-16" />
    </div>
  </Modal>
{:else}
  <Modal
    open={!isSaveTagOpen}
    size="xs"
    title="Add New Assets"
    classDialog="absolute max-h-screen"
    on:close={isSaveTagOpen ? undefined : onClose}
  >
    <form class="flex flex-col items-start gap-4 pb-44">
      <div class="flex">
        <MultiSelect
          options={coinOptions}
          selected={[insertedAsset.coin]}
          maxSelect={1}
          minSelect={1}
          maxOptions={7}
          ulSelectedClass="!w-32"
          outerDivClass="!rounded-r-none"
          liSelectedClass="!bg-transparent"
          ulOptionsClass="!rounded-tr-none"
          on:change={onInsertedAssetCoinChange(insertedAsset)}
        />
        <Input
          bind:value={insertedAsset.quantity}
          type="text"
          class="rounded-l-none"
        />
      </div>
      <MultiSelect
        let:option={tagOption}
        options={tagOptions}
        selected={insertedTag
          ? [
              {
                value: insertedTag.id,
                label: insertedTag.name,
              },
            ]
          : []}
        placeholder="Add tag"
        maxSelect={1}
        minSelect={1}
        maxOptions={4}
        inputClass="h-10"
        liOptionClass="!p-0"
        liSelectedClass="!bg-transparent"
        on:change={onInsertedTagSelect}
      >
        {#if tagOption.value === 'new_tag'}
          <div class="flex gap-2 border-b p-2">
            <FolderPlus />
            {tagOption.label}
          </div>
        {:else}
          <div class="flex gap-2 p-2">
            <FolderClosed />
            {tagOption.label}
          </div>
        {/if}
        <div
          slot="expand-icon"
          class={insertedTag ? '' : 'p-2'}
        >
          {#if !insertedTag}
            <FolderClosed />
          {/if}
        </div>
      </MultiSelect>
    </form>
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
        disabled={isUpdateDisabled}
        color="primary"
        on:click={() => {
          isLoading = true;
          onUpdate({
            tag_ids: insertedTag ? [insertedTag.id] : [],
            asset: {
              id: insertedAsset.id,
              coin: insertedAsset.coin,
              quantity: parseFloat(insertedAsset.quantity),
            },
          }).finally(() => {
            isLoading = false;
          });
        }}
      >
        Save Changes
      </Button>
    </div>
  </Modal>
{/if}
