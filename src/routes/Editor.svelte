<script lang="ts">
  import { t } from '../lib/i18n';
  import { getParams, getPath, ensureRouter } from '../lib/stores/router.svelte';

  ensureRouter();

  const params = $derived.by(() => {
    const p = getPath();
    return getParams(p, '/editor/:id');
  });
</script>

<div class="mx-auto max-w-3xl">
  <h1 class="text-2xl font-bold text-[var(--color-text)]">{t('nav.new_episode')}</h1>
  <p class="mt-2 text-sm text-[var(--color-text-muted)]">
    T1.6 placeholder — el editor de episodios (sidebar + Tiptap) se implementa en T1.5.5 y T1.5.6.
  </p>
  {#if params.id}
    <p class="mt-4 rounded-md border border-[var(--color-border)] bg-[var(--color-bg)] p-3 text-sm text-[var(--color-text-muted)]">
      Modo edición: <code class="font-mono text-[var(--color-accent-600)]">{params.id}</code>
    </p>
  {/if}
</div>
