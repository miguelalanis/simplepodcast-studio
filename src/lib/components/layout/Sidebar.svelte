<script lang="ts">
  import { Home, PenSquare, Settings as SettingsIcon } from 'lucide-svelte';
  import { t } from '../../i18n';
  import { getPath, navigate } from '../../stores/router.svelte';

  const navLinks = [
    { href: '/', labelKey: 'nav.dashboard', icon: Home },
    { href: '/editor', labelKey: 'nav.new_episode', icon: PenSquare },
    { href: '/settings', labelKey: 'nav.settings', icon: SettingsIcon },
  ];

  const isActive = (href: string) => {
    const path = getPath();
    if (href === '/') return path === '/';
    return path.startsWith(href);
  };
</script>

<aside class="hidden w-56 flex-shrink-0 border-r border-[var(--color-border)] bg-[var(--color-bg)] p-4 md:block">
  <nav class="flex flex-col gap-1">
    {#each navLinks as navLink (navLink.href)}
      <a
        href={`#${navLink.href}`}
        onclick={() => navigate(navLink.href)}
        class:list={[
          'flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium transition-colors',
          isActive(navLink.href)
            ? 'bg-[var(--color-bg-subtle)] text-[var(--color-text)]'
            : 'text-[var(--color-text-muted)] hover:bg-[var(--color-bg-subtle)] hover:text-[var(--color-text)]',
        ]}
      >
        <svelte:component this={navLink.icon} class="h-4 w-4" aria-hidden="true" />
        {t(navLink.labelKey)}
      </a>
    {/each}
  </nav>
</aside>
