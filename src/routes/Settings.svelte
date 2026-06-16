<script lang="ts">
  import { onMount } from 'svelte';
  import { Save, X } from '@lucide/svelte';
  import { t } from '../lib/i18n';
  import { getSettings, setSettings, clearCredentials, getSettingsPath, type AppConfig } from '../lib/stores/settings.svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Card from '$lib/components/ui/Card.svelte';

  let config: AppConfig | null = $state(null);
  let path: string = $state('');
  let saving = $state(false);
  let saved = $state(false);
  let clearing = $state(false);

  onMount(async () => {
    config = await getSettings();
    path = await getSettingsPath();
  });

  async function save() {
    if (!config) return;
    saving = true;
    saved = false;
    try {
      await setSettings(config);
      saved = true;
      setTimeout(() => (saved = false), 2000);
    } finally {
      saving = false;
    }
  }

  async function clear() {
    clearing = true;
    try {
      await clearCredentials();
      saved = true;
      setTimeout(() => (saved = false), 2000);
    } finally {
      clearing = false;
    }
  }
</script>

<div class="mx-auto max-w-3xl space-y-6">
  <header>
    <h1 class="text-2xl font-bold text-[var(--foreground)]">{t('settings.title')}</h1>
    <p class="mt-1 text-sm text-[var(--muted-foreground)]">
      Preferencias persistidas en <code class="rounded bg-[var(--muted)] px-1.5 py-0.5 font-mono text-xs">{path || '...'}</code>
    </p>
  </header>

  {#if config}
    <Card class="p-6 space-y-4">
      <h2 class="text-sm font-semibold uppercase tracking-wider text-[var(--muted-foreground)]">
        {t('settings.preferences')}
      </h2>

      <label class="block">
        <span class="text-sm font-medium text-[var(--foreground)]">Carpeta raíz</span>
        <Input
          type="text"
          bind:value={config.preferences.cloneRoot}
          placeholder="~/SimplePodcast"
          class="mt-1"
        />
        <span class="mt-1 block text-xs text-[var(--muted-foreground)]">
          Por defecto: <code>~/SimplePodcast</code>. Acá se clonan los repos.
        </span>
      </label>

      <div class="flex items-center gap-2 pt-2">
        <Button onclick={save} disabled={saving}>
          <Save class="h-4 w-4" />
          {saving ? t('common.loading') : t('common.save')}
        </Button>
        {#if saved}
          <span class="text-sm text-green-600 dark:text-green-400">{t('common.success')}</span>
        {/if}
      </div>
    </Card>

    <Card class="p-6 space-y-4">
      <h2 class="text-sm font-semibold uppercase tracking-wider text-[var(--muted-foreground)]">
        Credenciales
      </h2>
      <p class="text-sm text-[var(--muted-foreground)]">
        Los tokens de GitHub, Cloudflare y R2 se guardan en el keychain del SO.
      </p>
      <div class="flex items-center gap-2">
        <Button variant="destructive" onclick={clear} disabled={clearing}>
          <X class="h-4 w-4" />
          {clearing ? t('common.loading') : t('settings.clear_credentials')}
        </Button>
        {#if saved}
          <span class="text-sm text-green-600 dark:text-green-400">Listo</span>
        {/if}
      </div>
    </Card>

    <Card class="p-6">
      <h2 class="text-sm font-semibold uppercase tracking-wider text-[var(--muted-foreground)]">
        {t('settings.about')}
      </h2>
      <dl class="mt-3 space-y-1 text-sm text-[var(--muted-foreground)]">
        <div class="flex justify-between">
          <dt>{t('settings.version')}</dt>
          <dd class="font-mono text-[var(--foreground)]">0.1.0</dd>
        </div>
        <div class="flex justify-between">
          <dt>Repo</dt>
          <dd>
            <a
              href="https://github.com/miguelalanis/simplepodcast-studio"
              target="_blank"
              rel="noopener noreferrer"
              class="text-[var(--primary)] hover:underline"
            >
              miguelalanis/simplepodcast-studio
            </a>
          </dd>
        </div>
      </dl>
    </Card>
  {:else}
    <p class="text-sm text-[var(--muted-foreground)]">{t('common.loading')}</p>
  {/if}
</div>
