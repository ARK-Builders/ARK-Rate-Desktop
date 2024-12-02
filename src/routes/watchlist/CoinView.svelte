<script lang="ts">
  import { onMount } from 'svelte';

  export let coin: string;

  let coinImageSrc: string | undefined;

  // TODO: possibly extract it to an util function
  const getCoinImageSrc = async (coin: string): Promise<string | undefined> => {
    const src = `/images/coin/${coin}.svg`;
    return fetch(src)
      .then((response) => {
        if (response.ok) {
          return src;
        }
        return undefined;
      })
      .catch((err) => {
        console.error(err);
        return undefined;
      });
  };

  onMount(() => {
    getCoinImageSrc(coin).then((src) => {
      coinImageSrc = src;
    });
  });
</script>

<div class="flex items-center gap-2 font-normal">
  <div class="size-14 flex-shrink-0 overflow-hidden rounded-full">
    {#if coinImageSrc}
      <img
        src={coinImageSrc}
        alt="{coin} Logo"
        class="size-full"
      />
    {:else}
      <div class="flex size-full items-center justify-center bg-gray-600 text-center">
        <p class="text-xs text-white">{coin}</p>
      </div>
    {/if}
  </div>
  <p class="truncate font-bold">{coin}</p>
</div>
