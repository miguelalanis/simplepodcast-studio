<script lang="ts">
  import { t } from '../lib/i18n';
</script>

<div class="mx-auto max-w-3xl">
  <h1 class="text-2xl font-bold text-[var(--color-text)]">{t('nav.dashboard')}</h1>
  <p class="mt-2 text-sm text-[var(--color-text-muted)]">
    T1.6 placeholder — el dashboard real se implementa en T1.5.4.
  </p>
  <div class="mt-8 rounded-2xl border border-dashed border-[var(--color-border)] p-12 text-center">
    <p class="text-sm text-[var(--color-text-muted)]">
      {t('dashboard.no_episodes', { action: t('nav.new_episode') })}
    </p>
  </div>
</div>
