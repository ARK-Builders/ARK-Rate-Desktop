<script lang="ts">
  import { toasts, type ToastProperties } from '$lib/ui/global/stores/toastStore';
  import { Toast } from 'flowbite-svelte';
  import { CircleAlert, CircleHelp, Clock, Stars, TriangleAlert } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { fly } from 'svelte/transition';

  export let id: string;
  export let message: string;
  export let defaultTimeout = 6;
  export let type: ToastProperties['type'];

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

  onMount(() => {
    let counter = defaultTimeout;
    const t = setInterval(() => {
      counter--;
      if (counter < 1) {
        toasts.update((ts) => {
          return ts.filter((t) => t.id !== id);
        });
      }
    }, 1000);
    return () => {
      clearTimeout(t);
    };
  });
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
