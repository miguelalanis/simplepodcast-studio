<script lang="ts">
  import { onMount } from 'svelte';
  import { t } from '../lib/i18n';
  import { ensureRouter } from '../lib/stores/router.svelte';

  let loaded = $state(false);

  onMount(() => {
    ensureRouter();
    loaded = true;
  });
</script>

<div class="mx-auto max-w-3xl">
  <h1 class="text-2xl font-bold text-[var(--color-text)]">{t('settings.title')}</h1>
  <p class="mt-2 text-sm text-[var(--color-text-muted)]">
    T1.4 / T1.5.15 placeholder — la página real con credenciales de GitHub/Cloudflare se implementa más adelante.
  </p>

  {#if loaded}
    <div class="mt-8 space-y-6">
      <section>
        <h2 class="text-sm font-semibold uppercase tracking-wider text-[var(--color-text-muted)]">
          {t('settings.preferences')}
        </h2>
        <p class="mt-2 text-sm text-[var(--color-text-muted)]">
          Tema, idioma y carpeta raíz se persisten en <code class="font-mono text-xs">~/.simplepodcast/config.json</code>.
        </p>
      </section>
    </div>
  {/if}
</div>
