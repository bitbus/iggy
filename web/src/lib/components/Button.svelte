<!-- @migration-task Error while migrating Svelte code: This migration would change the name of a slot making the component unusable -->
<!-- @migration-task Error while migrating Svelte code: This migration would change the name of a slot making the component unusable -->
<!-- // TODO: check it - Error while migrating Svelte code: This migration would change the name of a slot making the component unusable -->
<script lang="ts">
  import { tooltip } from '$lib/actions/tooltip';
  import type { Placement } from '@floating-ui/dom';
  import type { HTMLButtonAttributes } from 'svelte/elements';
  import { twMerge } from 'tailwind-merge';

  const baseClasses =
    'flex items-center justify-center font-semibold transition-all gap-2 rounded-[4px] focus:outline-none focus-visible:ring focus:ring-blue-600/60 ring-offset-2 ring-offset-white dark:ring-offset-shadeD700  ';

  const disabledClasses =
    'disabled:bg-zinc-300  dark:disabled:bg-zinc-500 disabled:text-zinc-700 disabled:pointer-events-none';

  const variants = {
    rounded:
      'w-[40px] h-[40px] rounded-full dark:text-white flex items-center justify-center dark:hover:bg-shadeD300 hover:bg-shadeL500',
    outlined:
      'border-black dark:border-white dark:text-white border-2 bg-transparent dark:hover:bg-shadeD400 hover:bg-shadeL400',
    outlinedRed: 'border-2 border-red-500 text-red-500 hover:text-red-600 hover:border-red-600 ',
    contained:
      'bg-black hover:bg-shadeD600 text-white dark:bg-white dark:text-black dark:text-black hover:shadow-lg dark:shadow-shadeD300',
    containedRed: 'bg-red-500 hover:bg-red-600  text-white hover:shadow-lg',
    text: 'bg-transparent text-color enabled:dark:hover:bg-shadeD500 hover:bg-shadeL400'
  };

  const sizes = {
    xs: 'px-2 py-[4px] text-[10px]',
    sm: 'px-3 py-[6px] text-xs',
    md: 'px-5 py-2 text-sm',
    lg: 'px-6 py-3 text-base',
    xl: 'px-7 py-4 text-lg'
  };

  interface $$Props extends HTMLButtonAttributes {
    variant: keyof typeof variants;
    tooltipPlacement?: Placement;
    size?: keyof typeof sizes;
    class?: string;
  }

  export let variant: keyof typeof variants;
  export let tooltipPlacement: Placement = 'right';
  export let size: keyof typeof sizes = 'md';
  let className = '';
  export { className as class };
</script>

<!-- <div use:tooltip={{ placement: tooltipPlacement }} class="w-full">
  <button
    on:click
    data-trigger
    class={twMerge(baseClasses, disabledClasses, variants[variant], sizes[size], className)}
    {...$$restProps}
  >
    <slot />
  </button>

  {#if $$slots.tooltip}
    <div role="tooltip" class="tooltip">
      <slot name="tooltip" />
    </div>
  {/if}
</div> -->

<button
  on:click
  data-trigger
  use:tooltip={{ placement: tooltipPlacement, isTrigger: true }}
  class={twMerge(baseClasses, variants[variant], sizes[size], disabledClasses, className, ' ')}
  {...$$restProps}
>
  <slot />

  {#if $$slots.tooltip}
    <div role="tooltip" class="tooltip ring-bl">
      <slot name="tooltip" />
    </div>
  {/if}
</button>
