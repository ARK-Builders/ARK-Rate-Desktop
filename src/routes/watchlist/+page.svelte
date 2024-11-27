<script lang="ts">
  import { Button, Heading } from 'flowbite-svelte';
  import { ArrowRightLeft, EllipsisVertical, PlusCircle } from 'lucide-svelte';
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
        <tr class="border-t">
          <th>
            <Button
              color="none"
              class="flex w-max items-center gap-2 font-bold text-green-500"
            >
              Sort
              <ArrowRightLeft class="size-6" />
            </Button>
          </th>
          <th class="border-l">
            <!-- COIN -->
            <div class="flex items-center gap-2 px-6 py-4">
              <div class="size-16 overflow-hidden rounded-full border-2 border-white">
                <div class="flex size-full items-center justify-center bg-gray-600 text-center">
                  <p class="text-xs text-white">BTC</p>
                </div>
              </div>
              <p class="font-bold">BTC</p>
            </div>
          </th>
          <th class="w-full border-l">
            <Button
              color="none"
              class="flex w-max items-center gap-2 pr-0 font-bold text-green-500"
            >
              <PlusCircle class="size-6 fill-green-500 text-white" />
              Add currency
            </Button>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr class="border-t">
          <td>
            <div class="flex items-center gap-2 px-6 py-4">
              <div class="size-16 overflow-hidden rounded-full border-2 border-white">
                <div class="flex size-full items-center justify-center bg-gray-600 text-center">
                  <p class="text-xs text-white">BTC</p>
                </div>
              </div>
              <p class="font-bold">BTC</p>
            </div>
          </td>
          <td class="border-l">

          </td>
          <td class="border-l">

          </td>
        </tr>
      </tbody>
    </table>
  </div>
</div>
