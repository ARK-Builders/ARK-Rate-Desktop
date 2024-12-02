<script lang="ts">
  import { onMount } from 'svelte';

  export let coin: string;
  export let value: number;

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

<div class="flex gap-2">
  <div class="size-16 overflow-hidden rounded-full border-2 border-white">
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
  <div class="flex flex-col justify-center">
    <p>{coin}</p>
    <p class="text-gray-500 dark:text-gray-400">{value.toLocaleString()}</p>
  </div>
</div>
