<script lang="ts">
  import { Button, Heading, Hr, Input, Label, Modal } from 'flowbite-svelte';
  import { ArrowDownUp, ChevronUp, Coins, Pin, Plus, Trash } from 'lucide-svelte';
  import MultiSelect from 'svelte-multiselect';

  let isAddNewPairOpen = false;
</script>

<Modal
  bind:open={isAddNewPairOpen}
  size="xs"
  title="Add New Pair"
  classDialog="absolute"
>
  <form class="flex flex-col">
    <Label class="space-y-2">
      <span>From</span>
      <div class="flex">
        <MultiSelect
          options={[
            { value: 'BRL', label: 'BRL' },
            { value: 'USD', label: 'USD' },
            { value: 'CAD', label: 'CAD' },
          ]}
          maxSelect={1}
          minSelect={1}
          liSelectedClass="!bg-transparent"
          ulOptionsClass="!rounded-tr-none"
          ulSelectedClass="!flex-grow !w-full"
          outerDivClass="!rounded-r-none !w-24"
        />
        <Input
          type="text"
          class="rounded-l-none"
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
        <div class="flex items-center gap-4">
          <div class="flex flex-grow">
            <MultiSelect
              options={[
                { value: 'BRL', label: 'BRL' },
                { value: 'USD', label: 'USD' },
                { value: 'CAD', label: 'CAD' },
              ]}
              maxSelect={1}
              minSelect={1}
              liSelectedClass="!bg-transparent"
              ulOptionsClass="!rounded-tr-none"
              ulSelectedClass="!flex-grow !w-full"
              outerDivClass="!rounded-r-none !w-24"
            />
            <Input
              type="text"
              class="rounded-l-none"
            />
          </div>
          <Button
            outline
            size="xs"
            color="light"
          >
            <Trash class="size-4" />
          </Button>
        </div>
        <div class="flex items-center gap-4">
          <div class="flex flex-grow">
            <MultiSelect
              options={[
                { value: 'BRL', label: 'BRL' },
                { value: 'USD', label: 'USD' },
                { value: 'CAD', label: 'CAD' },
              ]}
              maxSelect={1}
              minSelect={1}
              liSelectedClass="!bg-transparent"
              ulOptionsClass="!rounded-tr-none"
              ulSelectedClass="!flex-grow !w-full"
              outerDivClass="!rounded-r-none !w-24"
            />
            <Input
              type="text"
              class="rounded-l-none"
            />
          </div>
          <Button
            outline
            size="xs"
            color="light"
          >
            <Trash class="size-4" />
          </Button>
        </div>
        <div class="flex justify-start">
          <Button
            size="xs"
            color="light"
            class="flex gap-2"
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
    <Button color="light">Cancel</Button>
    <Button color="primary">Save Changes</Button>
  </div>
</Modal>

<div class="h-full min-h-max w-full min-w-max overflow-auto p-24">
  {#if false}
    <div class="flex h-full w-full items-center justify-center">
      <div class="flex max-w-sm flex-col items-center gap-6">
        <div class="flex flex-col items-center gap-2 text-center">
          <Coins class="size-16 text-green-500" />
          <Heading tag="h4">Ready for calculation!</Heading>
          <p class="text-gray-500">
            Select your currencies and enter an amount to start converting. Your exchange results will appear here.
          </p>
        </div>
        <Button
          size="lg"
          color="primary"
          class="flex gap-1"
          on:click={() => (isAddNewPairOpen = true)}
        >
          <Plus />
          Calculate
        </Button>
      </div>
    </div>
  {:else}
    <div class="flex flex-col gap-16">
      <div class="flex items-center justify-between gap-12">
        <div class="flex gap-4">
          <Coins class="size-16 text-green-500" />
          <div class="flex flex-col gap-1">
            <Heading tag="h4">Ready for calculation!</Heading>
            <p class="text-gray-500">
              Select your currencies and enter an amount to start converting. Your exchange results will appear here.
            </p>
          </div>
        </div>
        <Button
          size="md"
          color="primary"
          class="flex gap-1"
          on:click={() => (isAddNewPairOpen = true)}
        >
          <Plus />
          Calculate
        </Button>
      </div>
      <div class="flex flex-col">
        <div class="flex items-center justify-between gap-12 border-b py-3 text-gray-500">
          <p class="font-bold">Pinned pairs</p>
          <Pin class="size-5 rotate-45 fill-gray-500" />
        </div>
        <!-- PAIR 6: three+, closed -->
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
      </div>
    </div>
  {/if}
</div>
