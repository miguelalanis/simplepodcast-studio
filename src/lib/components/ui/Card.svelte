<script lang="ts" module>
  import { tv } from 'tailwind-variants';

  export const cardVariants = tv({
    base: 'rounded-xl border border-[var(--border)] bg-[var(--card)] text-[var(--card-foreground)] shadow',
  });
</script>

<script lang="ts">
  import type { HTMLAttributes } from 'svelte/elements';
  import { cn } from '$lib/utils';

  type Props = HTMLAttributes<HTMLDivElement> & {
    class?: string;
  };
  let { class: className, children, ...rest }: Props = $props();
</script>

<div class={cn(cardVariants(), className)} {...rest}>
  {@render children?.()}
</div>
