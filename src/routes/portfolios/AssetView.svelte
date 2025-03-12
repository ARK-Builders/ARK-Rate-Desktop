<script lang="ts">
  import type { ViewPortfoliosResponse } from '$lib/business/interactors/view_portfolios/ViewPortfoliosResponse';
  import CoinView from './CoinView.svelte';

  type Asset = ViewPortfoliosResponse['portfolios'][0]['asset'];

  export let asset: Asset;
</script>

<div class="flex items-center justify-between border-b px-2 py-6 text-left">
  <CoinView
    coin={asset.coin}
    value={1 / asset.usd_value}
  />
  <div class="flex flex-col text-end">
    <p>${(asset.usd_value * asset.quantity).toLocaleString()}</p>
    <p class="text-gray-500 dark:text-gray-400">
      {asset.quantity.toLocaleString()}
      {asset.coin}
    </p>
  </div>
</div>
