<script lang="ts">
  export let coin: string;
</script>

<div class="flex items-center justify-center gap-2 px-6 py-4 font-normal">
  <div class="size-12 overflow-hidden rounded-full">
    <div class="flex size-full items-center justify-center bg-gray-600 text-center">
      <p class="text-xs text-white">{coin}</p>
    </div>
  </div>
  <p class="font-bold">{coin}</p>
</div>
