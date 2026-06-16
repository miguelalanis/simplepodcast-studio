<script lang="ts">
  import { Mic } from '@lucide/svelte';
  import ThemeToggle from '../common/ThemeToggle.svelte';
</script>

<header class="flex h-14 items-center justify-between border-b border-[var(--border)] bg-[var(--background)] px-6">
  <a href="/" class="flex items-center gap-3 group">
    <div class="flex h-9 w-9 items-center justify-center rounded-md bg-[var(--primary)] text-[var(--primary-foreground)]">
      <Mic class="h-4 w-4" aria-hidden="true" />
    </div>
    <span class="font-semibold text-[var(--foreground)] group-hover:text-[var(--primary)] transition-colors">
      Simple Podcast Studio
    </span>
  </a>

  <div class="flex items-center gap-3">
    <span class="text-sm text-[var(--muted-foreground)]">v0.1.0</span>
    <ThemeToggle />
  </div>
</header>
