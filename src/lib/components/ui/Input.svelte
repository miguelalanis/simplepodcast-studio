<script lang="ts" module>
  import { tv, type VariantProps } from 'tailwind-variants';

  export const inputVariants = tv({
    base: 'flex h-9 w-full rounded-md border border-[var(--border)] bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-[var(--muted-foreground)] focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-[var(--ring)] disabled:cursor-not-allowed disabled:opacity-50',
    variants: {
      size: {
        default: 'h-9',
        sm: 'h-8',
        lg: 'h-10',
      },
    },
    defaultVariants: {
      size: 'default',
    },
  });

  export type InputSize = NonNullable<VariantProps<typeof inputVariants>['size']>;
</script>

<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements';
  import { cn } from '$lib/utils';

  type Props = HTMLInputAttributes & {
    class?: string;
    size?: InputSize;
    value?: string | number;
  };

  let { class: className, size, ...rest }: Props = $props();
  let value: string | number = $state((rest.value as string | number) ?? '');
  const inputClass = $derived(cn(inputVariants({ size: size ?? 'default' }), className));
</script>

<input class={inputClass} bind:value {...rest} />
