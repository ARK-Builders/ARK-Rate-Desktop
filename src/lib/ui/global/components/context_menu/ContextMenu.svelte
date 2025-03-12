<script lang="ts">
  import { type IconProps } from 'lucide-svelte';
  import type { SvelteComponent } from 'svelte';

  type MenuItem = {
    label: string;
    class?: string;
    onClick: () => void;
    icon: typeof SvelteComponent<IconProps>;
  };

  export let position: {
    x: number;
    y: number;
  };
  export let menuItems: MenuItem[];
</script>

<nav
  class="fixed shadow"
  style="top:{position.y}px; left:{position.x}px"
>
  <div class="flex flex-col overflow-hidden rounded border bg-white">
    <ul>
      {#each menuItems as item}
        <li class="text-sm text-gray-800 hover:bg-gray-50">
          <button
            class="flex size-full items-center gap-4 p-4 text-left focus:outline-none {item.class}"
            on:click={item.onClick}
          >
            <svelte:component
              this={item.icon}
              class="size-4"
            />
            {item.label}
          </button>
        </li>
      {/each}
    </ul>
  </div>
</nav>
