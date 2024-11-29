<script lang="ts">
  export let coin: string;
  if (coin === 'SOLVBTC') {
    // coin = 'laksjdfalskdjfsdf';
  }
</script>

<div class="flex items-center gap-2 font-normal">
  <div class="size-14 flex-shrink-0 overflow-hidden rounded-full">
    <div class="flex size-full items-center justify-center bg-gray-600 text-center">
      <p class="text-xs text-white">{coin}</p>
    </div>
  </div>
  <p class="truncate font-bold">{coin}</p>
</div>
