<script lang="ts">
  import type { PairGroup } from '$lib/business/entities/PairGroup';
  import AddNewPairGroupModal from './AddNewPairGroupModal.svelte';
  import EmptyView from './EmptyView.svelte';
  import FilledView from './FilledView.svelte';

  let isAddNewPairGroupOpen = false;

  let pinnedPairGroups: PairGroup[] = [];
  let calculatedPairGroups: PairGroup[] = [];
  let frequentPairGroups: PairGroup[] = [];

  const onCalculateClick = () => (isAddNewPairGroupOpen = true);
</script>

<AddNewPairGroupModal bind:isOpen={isAddNewPairGroupOpen} />

<div class="h-full min-h-max w-full min-w-max overflow-auto p-24">
  {#if pinnedPairGroups.length === 0 && calculatedPairGroups.length === 0 && frequentPairGroups.length === 0}
    <EmptyView {onCalculateClick} />
  {:else}
    <FilledView
      {pinnedPairGroups}
      {calculatedPairGroups}
      {frequentPairGroups}
      {onCalculateClick}
    />
  {/if}
</div>
