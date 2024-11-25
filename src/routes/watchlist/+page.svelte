<script>
  import { Button, Heading } from 'flowbite-svelte';
  import { EllipsisVertical } from 'lucide-svelte';
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
    <table class="conteudo">
      <!-- TODO -->
    </table>
  </div>
</div>
