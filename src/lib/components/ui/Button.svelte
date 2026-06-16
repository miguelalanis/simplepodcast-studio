<script lang="ts" module>
  import type { ClassValue } from 'clsx';
  import { tv, type VariantProps } from 'tailwind-variants';

  export const buttonVariants = tv({
    base: 'inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-[var(--ring)] disabled:pointer-events-none disabled:opacity-50',
    variants: {
      variant: {
        default: 'bg-[var(--primary)] text-[var(--primary-foreground)] shadow hover:opacity-90',
        destructive: 'bg-[var(--destructive)] text-[var(--destructive-foreground)] shadow-sm hover:opacity-90',
        outline: 'border border-[var(--border)] bg-[var(--background)] shadow-sm hover:bg-[var(--accent)] hover:text-[var(--accent-foreground)]',
        secondary: 'bg-[var(--secondary)] text-[var(--secondary-foreground)] shadow-sm hover:opacity-80',
        ghost: 'hover:bg-[var(--accent)] hover:text-[var(--accent-foreground)]',
        link: 'text-[var(--primary)] underline-offset-4 hover:underline',
      },
      size: {
        default: 'h-9 px-4 py-2',
        sm: 'h-8 rounded-md px-3 text-xs',
        lg: 'h-10 rounded-md px-8',
        icon: 'h-9 w-9',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'default',
    },
  });

  export type ButtonVariant = NonNullable<VariantProps<typeof buttonVariants>['variant']>;
  export type ButtonSize = NonNullable<VariantProps<typeof buttonVariants>['size']>;
</script>

<script lang="ts">
  import type { HTMLButtonAttributes, HTMLAnchorAttributes } from 'svelte/elements';
  import { cn } from '$lib/utils';

  type Props = (HTMLButtonAttributes | HTMLAnchorAttributes) & {
    variant?: ButtonVariant;
    size?: ButtonSize;
    class?: ClassValue;
    href?: string;
  };

  let {
    variant = "default",
    size = "default",
    class: className,
    href,
    type = "button",
    children,
    ...rest
  }: Props = $props();
</script>

{#if href}
  <a {href} class={cn(buttonVariants({ variant: variant ?? 'default', size: size ?? 'default' }), className)} {...rest as HTMLAnchorAttributes}>
    {@render children?.()}
  </a>
{:else}
  <button type={type as 'button' | 'submit' | 'reset' | undefined} class={cn(buttonVariants({ variant: variant ?? 'default', size: size ?? 'default' }), className)} {...rest as HTMLButtonAttributes}>
    {@render children?.()}
  </button>
{/if}
