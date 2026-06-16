<script lang="ts">
  import { onMount } from 'svelte';
  import { Sun, Moon, Monitor } from 'lucide-svelte';

  type Theme = 'light' | 'dark' | 'system';
  let theme: Theme = $state('system');

  function getCurrentTheme(): Theme {
    try {
      const stored = localStorage.getItem('theme');
      if (stored === 'light' || stored === 'dark' || stored === 'system') return stored;
    } catch {
      // localStorage puede no estar disponible
    }
    return 'system';
  }

  function applyTheme(next: Theme) {
    theme = next;
    if (next === 'system') {
      document.documentElement.removeAttribute('data-theme');
      try { localStorage.setItem('theme', 'system'); } catch { /* noop */ }
    } else {
      document.documentElement.setAttribute('data-theme', next);
      try { localStorage.setItem('theme', next); } catch { /* noop */ }
    }
  }

  onMount(() => {
    theme = getCurrentTheme();
  });
</script>

<div role="group" aria-label="Tema" class="inline-flex items-center gap-0.5 rounded-full border border-[var(--color-border)] bg-[var(--color-bg)] p-0.5">
  <button
    type="button"
    onclick={() => applyTheme('light')}
    aria-label="Tema claro"
    aria-pressed={theme === 'light'}
    class:list={[
      'inline-flex h-7 w-7 items-center justify-center rounded-full transition-colors',
      theme === 'light' ? 'bg-[var(--color-bg-subtle)] text-[var(--color-text)]' : 'text-[var(--color-text-muted)]',
    ]}
  >
    <Sun class="h-3.5 w-3.5" aria-hidden="true" />
  </button>
  <button
    type="button"
    onclick={() => applyTheme('system')}
    aria-label="Tema del sistema"
    aria-pressed={theme === 'system'}
    class:list={[
      'inline-flex h-7 w-7 items-center justify-center rounded-full transition-colors',
      theme === 'system' ? 'bg-[var(--color-bg-subtle)] text-[var(--color-text)]' : 'text-[var(--color-text-muted)]',
    ]}
  >
    <Monitor class="h-3.5 w-3.5" aria-hidden="true" />
  </button>
  <button
    type="button"
    onclick={() => applyTheme('dark')}
    aria-label="Tema oscuro"
    aria-pressed={theme === 'dark'}
    class:list={[
      'inline-flex h-7 w-7 items-center justify-center rounded-full transition-colors',
      theme === 'dark' ? 'bg-[var(--color-bg-subtle)] text-[var(--color-text)]' : 'text-[var(--color-text-muted)]',
    ]}
  >
    <Moon class="h-3.5 w-3.5" aria-hidden="true" />
  </button>
</div>
