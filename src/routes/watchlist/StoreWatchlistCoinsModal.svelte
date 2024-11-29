<script lang="ts">
  import type { StoreWatchlistCoinsRequest } from '$lib/business/interactors/store_watchlist_coins/StoreWatchlistCoinsRequest';
  import { Button, Modal, Spinner } from 'flowbite-svelte';
  import { Plus, Trash } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import MultiSelect from 'svelte-multiselect';

  export let coins: string[];

  export let onClose: () => void;
  export let onStore: (request: StoreWatchlistCoinsRequest) => Promise<void>;

  let isLoading = false;
  let isDisabled = false;
  let isAddCoinDisabled = false;

  let insertedCoins: string[] = [];

  const getRandomCoin = (): string => {
    const nonRepeatedCoins = coins.filter((c) => insertedCoins.findIndex((ic) => ic === c) === -1);
    const randomIndex = Math.floor(Math.random() * nonRepeatedCoins.length);
    return nonRepeatedCoins[randomIndex];
  };

  const addCoin = () => {
    insertedCoins = [...insertedCoins, getRandomCoin()];
  };

  const removeCoin = (i: number) => {
    insertedCoins = insertedCoins.filter((_, j) => j !== i);
  };

  const onCoinChange = (i: number) => (event: CustomEvent) => {
    const detail = event.detail;
    if (!detail) return;
    if (detail.type === 'remove') {
      isAddCoinDisabled = true;
      return;
    }
    isAddCoinDisabled = false;
    insertedCoins[i] = detail.option;
  };

  onMount(() => {
    isLoading = false;
    addCoin();
  });
</script>

{#if isLoading}
  <Modal
    dismissable={false}
    open
    size="xs"
    title="Add Currencies"
    classDialog="absolute max-h-screen"
    on:close={onClose}
  >
    <div class="flex size-full items-center justify-center">
      <Spinner class="size-16" />
    </div>
  </Modal>
{:else}
  <Modal
    open
    size="xs"
    title="Add Currencies"
    classDialog="absolute max-h-screen"
    on:close={onClose}
  >
    <form class="pb-44">
      <div class="flex flex-col gap-4">
        {#each insertedCoins as coin, i}
          <div class="flex items-center gap-4">
            <MultiSelect
              options={coins}
              disabled={isDisabled}
              selected={[coin]}
              maxSelect={1}
              minSelect={1}
              maxOptions={7}
              outerDivClass="flex flex-grow"
              liSelectedClass="!bg-transparent"
              on:change={onCoinChange(i)}
            />
            {#if i > 0}
              <Button
                outline
                size="xs"
                color="light"
                on:click={() => removeCoin(i)}
              >
                <Trash class="size-4" />
              </Button>
            {/if}
          </div>
        {/each}
        <div class="flex justify-start">
          <Button
            disabled={isDisabled || isAddCoinDisabled}
            size="xs"
            color="light"
            class="flex gap-2"
            on:click={addCoin}
          >
            <Plus class="size-4" />
            New Currency
          </Button>
        </div>
      </div>
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
        disabled={isDisabled}
        color="primary"
        on:click={() => {
          isLoading = true;
          onStore({
            coins: insertedCoins,
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
