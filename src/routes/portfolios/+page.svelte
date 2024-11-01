<script lang="ts">
  import { Spinner } from 'flowbite-svelte';
  import EmptyView from './EmptyView.svelte';

  let isLoading = false;

  const onSaveAssetOpen = () => {
    
  }
</script>

{#if isLoading}
  <div class="flex size-full items-center justify-center">
    <Spinner class="size-32" />
  </div>
{:else}
  <div class="h-full min-h-max w-full min-w-max overflow-auto p-24">
    {#if true}
      <EmptyView {onSaveAssetOpen} />
    {:else}
      <!-- PASS -->
    {/if}
  </div>
{/if}
