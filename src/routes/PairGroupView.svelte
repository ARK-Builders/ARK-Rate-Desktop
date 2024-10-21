<script lang="ts">
  import type { PairGroup } from '$lib/business/entities/PairGroup';
  import { ChevronDown, ChevronUp } from 'lucide-svelte';
  import { DateTime, Duration } from 'luxon';

  export let pairGroup: PairGroup;

  let isOpen = false;

  const getHeading = (): string => {
    let comparisonHeading;
    if (pairGroup.pairs.length === 1) {
      comparisonHeading = pairGroup.pairs[0].comparison;
    } else if (pairGroup.pairs.length > 3) {
      comparisonHeading = `${pairGroup.pairs
        .slice(0, 3)
        .map((p) => p.comparison)
        .join(', ')} and ${pairGroup.pairs.length - 3}+`;
    } else {
      comparisonHeading = `${pairGroup.pairs
        .slice(0, -1)
        .map((p) => p.comparison)
        .join(', ')} and ${pairGroup.pairs.at(-1)?.comparison}`;
    }
    return `${pairGroup.pairs[0].base} to ${comparisonHeading}`;
  };

  const getEquivalentValueHeading = (): string => {
    return `${pairGroup.pairs[0].value} ${pairGroup.pairs[0].comparison}`;
  };

  const getLastUpdateMessage = (): string => {
    const now = DateTime.now();
    const lastUpdate = DateTime.fromJSDate(pairGroup.updatedAt);
    const duration = Duration.fromMillis(now.toMillis() - lastUpdate.toMillis());
    return duration.shiftTo('minutes').toHuman({
      listStyle: 'narrow',
      unitDisplay: 'narrow',
      roundingMode: 'floor',
      maximumFractionDigits: 0,
    });
  };
</script>

<button
  class="flex w-full gap-2 border-b px-2 py-6 text-start hover:cursor-pointer"
  on:click={() => (isOpen = !isOpen)}
>
  <div class="flex flex-grow-0">
    <div class="size-16 overflow-hidden rounded-full border-2 border-white">
      <div class="flex size-full items-center justify-center bg-gray-600 text-center">
        <p class="text-xs text-white">{pairGroup.pairs[0].base}</p>
      </div>
      <!-- <img
        alt="EUR Logo"
        class="size-full"
        src="images/fiat-currencies/EUR.png"
      /> -->
    </div>
    <div class="-ml-4 size-16 overflow-hidden rounded-full border-2 border-white{isOpen ? ' hidden' : ''}">
      <div class="flex size-full items-center justify-center bg-gray-200 text-center">
        {#if pairGroup.pairs.length > 1}
          <p class="text-lg text-gray-600">+{pairGroup.pairs.length}</p>
        {:else}
          <p class="text-sm text-gray-600">{pairGroup.pairs[0].comparison}</p>
        {/if}
      </div>
    </div>
  </div>
  <div class="flex flex-grow flex-col">
    <div class="flex items-center justify-between">
      <p>{getHeading()}</p>
      {#if isOpen}
        <ChevronUp class="size-6 text-gray-500" />
      {:else}
        <ChevronDown class="size-6 text-gray-500" />
      {/if}
    </div>
    <div class="flex items-end justify-between gap-4 text-gray-500">
      <div class="flex flex-col gap-2">
        <p>{pairGroup.multiplier} {pairGroup.pairs[0].base} = {isOpen ? '' : getEquivalentValueHeading()}</p>
        <div class="max-w-lg flex-wrap gap-2 {isOpen ? 'flex' : 'hidden'}">
          {#each pairGroup.pairs as pair}
            <div class="flex items-center gap-2">
              <div class="size-6 overflow-hidden rounded-full">
                <div class="min-h-full min-w-full bg-gray-400"></div>
                <!-- <img
                   class="size-full"
                   src="/images/fiat-currencies/USD.png"
                   alt="USD Logo"
                 /> -->
              </div>
              <p>{pair.value} {pair.comparison}</p>
            </div>
          {/each}
        </div>
      </div>
      <p>Last {pairGroup.isPinned ? 'refreshed' : 'updated'} {getLastUpdateMessage()} ago</p>
    </div>
  </div>
</button>
