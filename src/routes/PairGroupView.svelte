<script lang="ts">
  import { ChevronUp } from 'lucide-svelte';
</script>

<div class="flex w-full gap-2 border-b px-2 py-6 transition-all hover:cursor-pointer">
  <div class="flex flex-grow-0">
    <div class="size-12 overflow-hidden rounded-full border-2 border-white">
      <img
        alt="EUR Logo"
        class="size-full"
        src="images/fiat-currencies/EUR.png"
      />
    </div>
    <div class="-ml-4 hidden size-12 overflow-hidden rounded-full border-2 border-white">
      <div class="flex size-full items-center justify-center bg-gray-200 text-center">
        <p class="text-lg text-gray-600">+5</p>
      </div>
    </div>
  </div>
  <div class="flex flex-grow flex-col">
    <div class="flex items-center justify-between">
      <p>EUR to USD, BRL and 3+</p>
      <ChevronUp class="size-6 text-gray-500" />
    </div>
    <div class="flex items-end justify-between gap-4 text-gray-500">
      <div class="flex flex-col gap-2">
        <p>1 EUR = 1.09076 USD</p>
        <div class="flex max-w-lg flex-wrap gap-2">
          <div class="flex items-center gap-2">
            <div class="size-6 overflow-hidden rounded-full">
              <img
                class="size-full"
                src="/images/fiat-currencies/USD.png"
                alt="USD Logo"
              />
            </div>
            <p>1.09076 USD</p>
          </div>
          <div class="flex items-center gap-2">
            <div class="size-6 overflow-hidden rounded-full">
              <img
                class="size-full"
                src="/images/fiat-currencies/BRL.png"
                alt="BRL Logo"
              />
            </div>
            <p>5.09076 BRL</p>
          </div>
          <div class="flex items-center gap-2">
            <div class="size-6 overflow-hidden rounded-full">
              <img
                class="size-full"
                src="/images/fiat-currencies/JPY.png"
                alt="JPY Logo"
              />
            </div>
            <p>1230.09076 JPY</p>
          </div>
          <div class="flex items-center gap-2">
            <div class="size-6 overflow-hidden rounded-full">
              <img
                class="size-full"
                src="/images/fiat-currencies/MXN.png"
                alt="MXN Logo"
              />
            </div>
            <p>12340.09076 MXN</p>
          </div>
          <div class="flex items-center gap-2">
            <div class="size-6 overflow-hidden rounded-full">
              <img
                class="size-full"
                src="/images/fiat-currencies/CAD.png"
                alt="CAD Logo"
              />
            </div>
            <p>4.09076 CAD</p>
          </div>
        </div>
      </div>
      <p>Last refreshed 2 minutes ago</p>
    </div>
  </div>
</div>
