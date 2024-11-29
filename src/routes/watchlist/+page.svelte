<script lang="ts">
  import { Button, Heading } from 'flowbite-svelte';
  import { ArrowRightLeft, EllipsisVertical } from 'lucide-svelte';
  import CoinView from './CoinView.svelte';
</script>

<div class="h-full min-w-max overflow-auto p-24">
  <!-- COIN MATRIX -->
  <div class="rounded-lg border">
    <!-- HEADER -->
    <div class="relative p-4">
      <Heading tag="h5">Matrix - Multi-currency monitor</Heading>
      <p>Last refreshed 2 minutes ago.</p>
      <Button
        color="none"
        size="xs"
        class="absolute right-4 top-4"
      >
        <EllipsisVertical class="size-5 text-gray-500" />
      </Button>
    </div>
    <!-- BODY -->
    <table class="table-auto">
      <thead>
        <tr class="border-t bg-gray-100">
          <th>
            <Button
              color="none"
              class="mx-auto flex w-max items-center gap-2 font-bold text-green-500"
            >
              Sort
              <ArrowRightLeft class="size-5" />
            </Button>
          </th>
          <th class="border-l">
            <CoinView coin="BTC" />
          </th>
          <th class="border-l">
            <CoinView coin="USD" />
          </th>
          <th class="w-full border-l">
            <Button
              color="none"
              class="flex w-max items-center gap-2 pr-0 font-bold text-green-500"
            >
              <span class="flex size-5 items-center justify-center rounded-full bg-green-500 p-0.5 text-white">+</span>
              Add currency
            </Button>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr class="border-t">
          <td class="bg-gray-100">
            <CoinView coin="BTC" />
          </td>
          <td class="border-l text-center text-sm">1</td>
          <td class="border-l text-center text-sm">$93,579</td>
          <!-- placeholder -->
          <td class="border-l"></td>
        </tr>
        <tr class="border-t">
          <td class="bg-gray-100">
            <CoinView coin="USD" />
          </td>
          <td class="border-l text-center text-sm">$0.00001068</td>
          <td class="border-l text-center text-sm">1</td>
          <!-- placeholder -->
          <td class="border-l"></td>
        </tr>
      </tbody>
    </table>
  </div>
</div>
