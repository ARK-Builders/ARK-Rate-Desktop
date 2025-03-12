<script lang="ts">
  import { Spinner } from 'flowbite-svelte';
  import EmptyView from './EmptyView.svelte';
  import FilledView from './FilledView.svelte';

  let isLoading = false;

  const onAddAlertOpen = () => {
    // TODO
  };
</script>

{#if isLoading}
  <div class="flex size-full items-center justify-center">
    <Spinner class="size-32" />
  </div>
{:else}
  <div class="h-full min-h-max w-full min-w-max overflow-auto p-24">
    {#if true}
      <EmptyView {onAddAlertOpen} />
    {:else}
      <FilledView {onAddAlertOpen} />
    {/if}
  </div>
{/if}
