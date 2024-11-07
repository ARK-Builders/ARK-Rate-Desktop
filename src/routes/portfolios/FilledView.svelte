<script lang="ts">
  import { Button, Heading, TabItem, Tabs } from 'flowbite-svelte';
  import { ArrowUp, ChartNoAxesColumn, Plus } from 'lucide-svelte';
</script>

<div class="mb-16 flex items-center justify-between gap-12">
  <div class="flex gap-4">
    <ChartNoAxesColumn class="size-16 text-green-500" />
    <div class="flex flex-col gap-1">
      <Heading tag="h4">Track Your Assets</Heading>
      <p class="max-w-2xl text-wrap text-gray-500">
        Add currencies manualy to track and compare. Your portfolio helps you monitor and manage your favorite or
        frequently used currencies.
      </p>
    </div>
  </div>
  <Button
    size="md"
    color="primary"
    class="flex gap-1"
  >
    <Plus />
    New Asset
  </Button>
</div>
<Tabs
  tabStyle="underline"
  contentClass=""
>
  <TabItem
    open
    title="Companies"
  >
    <div class="flex flex-col">
      <div class="flex flex-col items-start gap-2 border-b px-2 py-10">
        <p class="text-gray-500 dark:text-gray-400">Total Assets</p>
        <div class="flex gap-0.5 align-top">
          <p class="text-2xl">$</p>
          <p class="text-4xl">5,000</p>
          <div class="flex text-green-500">
            <ArrowUp class="size-5" />
            <p>0.10%</p>
          </div>
        </div>
      </div>
      <div class="flex flex-col">
        <!-- Asset -->
        <div class="flex items-center justify-between border-b px-2 py-6">
          <div class="flex gap-2">
            <div class="size-16 overflow-hidden rounded-full border-2 border-white">
              <div class="flex size-full items-center justify-center bg-gray-600 text-center">
                <p class="text-xs text-white">USD</p>
              </div>
              <!-- <img
                   alt="EUR Logo"
                   class="size-full"
                   src="images/fiat-currencies/EUR.png"
               /> -->
            </div>
            <div class="flex flex-col justify-center">
              <p>USD United States Dollar</p>
              <p class="text-gray-500 dark:text-gray-400">1.00</p>
            </div>
          </div>
          <div class="flex flex-col text-end">
            <p>$1,000</p>
            <p class="text-gray-500 dark:text-gray-400">1,000 USD</p>
          </div>
        </div>
        <!--  -->
      </div>
    </div>
  </TabItem>
</Tabs>
