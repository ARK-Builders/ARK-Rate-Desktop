<script lang="ts">
  import type { SavePairGroupRequest } from '$lib/business/interactors/save_pair_group/SavePairGroupRequest';
  import type { ViewPairGroupsResponse } from '$lib/business/interactors/view_pair_groups/ViewPairGroupsResponse';
  import { Button, Hr, Input, Label, Modal, Spinner } from 'flowbite-svelte';
  import { ArrowDownUp, Plus, Trash } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import MultiSelect from 'svelte-multiselect';

  type USDPair = ViewPairGroupsResponse['usd_pairs'][0];
  type PairGroup = ViewPairGroupsResponse['pair_groups'][0];
  type Pair = PairGroup['pairs'][0];

  export let usdPairs: USDPair[];

  export let onClose: () => void;
  export let onSave: (request: SavePairGroupRequest) => Promise<void>;

  let isLoading = true;
  let isDisabled = false;
  let isAddPairDisabled = false;

  // DEPRECATED: it is better to use individual, isolated, variables as field values instead
  let pairGroup: PairGroup;
  let options: string[] = [];

  const getRandomBase = (): string => {
    const randomIndex = Math.floor(Math.random() * usdPairs.length);
    return usdPairs[randomIndex].comparison;
  };

  const getRandomComparison = (base: string): string => {
    const customUSDPairs = usdPairs.filter((p) => p.comparison !== base);
    const randomIndex = Math.floor(Math.random() * customUSDPairs.length);
    return customUSDPairs[randomIndex].comparison;
  };

  const getEquivalentUSDValue = (target: string): number => {
    for (const pair of usdPairs) {
      if (pair.comparison === target) {
        return 1 / pair.value;
      }
    }
    throw new Error(`Could not find an equivalent USD value for "${target}"`);
  };

  const addPair = () => {
    let base: string;
    if (pairGroup.pairs.length > 0) {
      base = pairGroup.pairs[0].base;
    } else {
      base = getRandomBase();
    }
    const comparison = getRandomComparison(base);
    const baseValue = getEquivalentUSDValue(base);
    const comparisonValue = getEquivalentUSDValue(comparison);
    pairGroup.pairs = [
      ...pairGroup.pairs,
      {
        id: crypto.randomUUID(),
        base,
        comparison,
        value: baseValue / comparisonValue,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      },
    ];
  };

  const onBaseChange = (event: CustomEvent) => {
    const detail = event.detail;
    if (!detail) return;
    if (detail.type === 'remove') {
      isDisabled = true;
      return;
    }
    isDisabled = false;
    const base = detail.option;
    const baseValue = getEquivalentUSDValue(base);
    pairGroup.pairs = pairGroup.pairs.map((p) => {
      const comparisonValue = getEquivalentUSDValue(p.comparison);
      return {
        ...p,
        base,
        value: baseValue / comparisonValue,
      };
    });
  };

  const onMultiplierKeyUp = (event: KeyboardEvent) => {
    const target = event.target as HTMLInputElement;
    if (target.value !== target.value.replace(/[^0-9.]/g, '')) {
      target.value = target.value.replace(/[^0-9.]/g, '');
    }
    // Ensures that only one decimal point is allowed
    if ((target.value.match(/\./g) || []).length > 1) {
      target.value = target.value.slice(0, target.value.lastIndexOf('.'));
    }
    const parts = target.value.split('.');
    if (parts.length > 1) {
      // Ensures that it won't remove decimals
      if (parts[1].length === 0 || parseInt(parts[1]) === 0) return;
    }
    const multiplier = target.value.length > 0 ? parseFloat(target.value) : 0;
    if (multiplier !== pairGroup.multiplier) {
      pairGroup.multiplier = multiplier;
    }
  };

  const removePair = (pair: Pair) => {
    pairGroup.pairs = pairGroup.pairs.filter((p) => p.id !== pair.id);
  };

  const onComparisonChange = (pair: Pair) => (event: CustomEvent) => {
    const detail = event.detail;
    if (!detail) return;
    if (detail.type === 'remove') {
      isAddPairDisabled = true;
      return;
    }
    isAddPairDisabled = false;
    const comparison = detail.option;
    pairGroup.pairs = pairGroup.pairs.map((p) => {
      if (p.id === pair.id) {
        const baseValue = getEquivalentUSDValue(p.base);
        const comparisonValue = getEquivalentUSDValue(comparison);
        return {
          ...p,
          comparison,
          value: baseValue / comparisonValue,
        };
      }
      return p;
    });
  };

  const onComparisonMultiplierKeyUp = (pair: Pair) => (event: KeyboardEvent) => {
    const target = event.target as HTMLInputElement;
    if (target.value !== target.value.replace(/[^0-9.]/g, '')) {
      target.value = target.value.replace(/[^0-9.]/g, '');
    }
    // Ensures that only one decimal point is allowed
    if ((target.value.match(/\./g) || []).length > 1) {
      target.value = target.value.slice(0, target.value.lastIndexOf('.'));
    }
    const parts = target.value.split('.');
    if (parts.length > 1) {
      // Ensures that it won't remove decimals
      if (parts[1].length === 0 || parseInt(parts[1]) === 0) return;
    }
    const multiplier = target.value.length > 0 ? parseFloat(target.value) / pair.value : 0;
    if (multiplier !== pairGroup.multiplier) {
      pairGroup.multiplier = multiplier;
    }
  };

  onMount(() => {
    pairGroup = {
      id: crypto.randomUUID(),
      multiplier: 1,
      is_pinned: false,
      pairs: [],
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };
    addPair();
    options = usdPairs.map((p) => p.comparison);
    isLoading = false;
  });
</script>

{#if isLoading || pairGroup.pairs.length < 1}
  <Modal
    dismissable={false}
    open
    size="xs"
    title="Add New Pair"
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
    title="Add New Pair"
    classDialog="absolute max-h-screen"
    on:close={onClose}
  >
    <form class="flex flex-col">
      <Label class="space-y-2">
        <span>From</span>
        <div class="flex">
          <MultiSelect
            {options}
            selected={[pairGroup.pairs[0].base]}
            maxSelect={1}
            minSelect={1}
            ulSelectedClass="!w-28"
            outerDivClass="!rounded-r-none"
            liSelectedClass="!bg-transparent"
            ulOptionsClass="!rounded-tr-none"
            on:change={onBaseChange}
          />
          <Input
            value={pairGroup.multiplier}
            type="text"
            class="rounded-l-none"
            on:keyup={onMultiplierKeyUp}
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
          {#each pairGroup.pairs as pair, i}
            <div class="flex items-center gap-4">
              <div class="flex flex-grow">
                <MultiSelect
                  {options}
                  maxSelect={1}
                  minSelect={1}
                  disabled={isDisabled}
                  selected={[pair.comparison]}
                  ulSelectedClass="!w-28"
                  outerDivClass="!rounded-r-none"
                  liSelectedClass="!bg-transparent"
                  ulOptionsClass="!rounded-tr-none"
                  on:change={onComparisonChange(pair)}
                />
                <Input
                  disabled={isDisabled}
                  value={pair.value * pairGroup.multiplier}
                  type="text"
                  class="rounded-l-none"
                  on:keyup={onComparisonMultiplierKeyUp(pair)}
                />
              </div>
              {#if i > 0}
                <Button
                  outline
                  size="xs"
                  color="light"
                  on:click={() => removePair(pair)}
                >
                  <Trash class="size-4" />
                </Button>
              {/if}
            </div>
          {/each}
          <div class="flex justify-start">
            <Button
              disabled={isDisabled || isAddPairDisabled}
              size="xs"
              color="light"
              class="flex gap-2"
              on:click={addPair}
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
          onSave({
            pair_group: {
              is_pinned: false,
              multiplier: pairGroup.multiplier,
              pairs: pairGroup.pairs.map((p) => ({
                base: p.base,
                value: p.value,
                comparison: p.comparison,
              })),
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
