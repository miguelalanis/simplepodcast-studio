<script lang="ts">
  import { Plus, Mic } from '@lucide/svelte';
  import { t } from '../lib/i18n';
  import { navigate } from '../lib/stores/router.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Card from '$lib/components/ui/Card.svelte';
</script>

<div class="mx-auto max-w-4xl space-y-6">
  <header class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-bold text-[var(--foreground)]">{t('nav.dashboard')}</h1>
      <p class="mt-1 text-sm text-[var(--muted-foreground)]">
        Gestión de episodios y configuración del podcast.
      </p>
    </div>
    <Button onclick={() => navigate('/editor')}>
      <Plus class="h-4 w-4" />
      {t('nav.new_episode')}
    </Button>
  </header>

  <Card class="p-12 text-center">
    <div class="flex flex-col items-center gap-3">
      <div class="flex h-14 w-14 items-center justify-center rounded-full bg-[var(--muted)]">
        <Mic class="h-6 w-6 text-[var(--muted-foreground)]" />
      </div>
      <h2 class="text-lg font-semibold text-[var(--foreground)]">
        {t('dashboard.no_episodes', { action: '' })}
      </h2>
      <p class="max-w-sm text-sm text-[var(--muted-foreground)]">
        Empezá creando tu primer episodio. El contenido se guarda en
        <code class="rounded bg-[var(--muted)] px-1.5 py-0.5 font-mono text-xs">src/content/episodes/</code>
        de tu repo.
      </p>
      <Button class="mt-2" onclick={() => navigate('/editor')}>
        <Plus class="h-4 w-4" />
        {t('nav.new_episode')}
      </Button>
    </div>
  </Card>
</div>
