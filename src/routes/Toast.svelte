<script lang="ts">
  import { Toast } from 'flowbite-svelte';
  import { CircleAlert, CircleHelp, Clock, Stars, TriangleAlert } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { fly } from 'svelte/transition';
  import { toasts, type ToastProperties } from './layoutStore';

  export let id: string;
  export let message: string;
  export let defaultTimeout = 6;
  export let type: ToastProperties['type'];

  let isCounting = true;
  let counter = defaultTimeout;
  function timeout() {
    counter--;
    if (counter > 0) {
      setTimeout(timeout, 1000);
    } else {
      isCounting = false;
      counter = defaultTimeout;
      $toasts = $toasts.filter((t) => t.id !== id);
    }
  }

  function getColor():
    | 'dark'
    | 'red'
    | 'yellow'
    | 'green'
    | 'indigo'
    | 'purple'
    | 'pink'
    | 'blue'
    | 'primary'
    | 'none' {
    switch (type) {
      case 'warn':
        return 'yellow';
      case 'success':
        return 'green';
      case 'error':
        return 'red';
      default:
        return 'dark';
    }
  }

  onMount(timeout);
</script>

<Toast
  transition={fly}
  color={getColor()}
  dismissable={false}
>
  <svelte:fragment slot="icon">
    {#if type === 'error'}
      <CircleAlert class="size-6" />
    {:else if type === 'warn'}
      <TriangleAlert class="size-6" />
    {:else if type === 'info'}
      <CircleHelp class="size-6" />
    {:else if type === 'wait'}
      <Clock class="size-6" />
    {:else if type === 'success'}
      <Stars class="size-6" />
    {/if}
  </svelte:fragment>
  {message}
</Toast>
