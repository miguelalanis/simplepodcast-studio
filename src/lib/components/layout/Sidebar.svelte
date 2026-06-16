<script lang="ts">
  import { Home, PenSquare, Settings as SettingsIcon, ChevronsLeft, ChevronsRight } from '@lucide/svelte';
  import { t } from '../../i18n';
  import { getPath, navigate } from '../../stores/router.svelte';

  const navLinks = [
    { href: '/', labelKey: 'nav.dashboard', icon: Home },
    { href: '/editor', labelKey: 'nav.new_episode', icon: PenSquare },
    { href: '/settings', labelKey: 'nav.settings', icon: SettingsIcon },
  ];

  let collapsed = $state(false);

  const isActive = (href: string) => {
    const path = getPath();
    if (href === '/') return path === '/';
    return path.startsWith(href);
  };
</script>

<aside
  class:list={[
    'flex flex-shrink-0 flex-col border-r border-[var(--border)] bg-[var(--background)] transition-all duration-200',
    collapsed ? 'w-14' : 'w-56',
  ]}
>
  <nav class="flex flex-1 flex-col gap-1 p-2">
    {#each navLinks as navLink (navLink.href)}
      <a
        href={`#${navLink.href}`}
        onclick={() => navigate(navLink.href)}
        title={collapsed ? t(navLink.labelKey) : undefined}
        class:list={[
          'flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium transition-colors',
          collapsed ? 'justify-center px-2' : '',
          isActive(navLink.href)
            ? 'bg-[var(--muted)] text-[var(--foreground)]'
            : 'text-[var(--muted-foreground)] hover:bg-[var(--muted)] hover:text-[var(--foreground)]',
        ]}
      >
        <svelte:component this={navLink.icon} class="h-4 w-4 flex-shrink-0" aria-hidden="true" />
        {#if !collapsed}
          <span>{t(navLink.labelKey)}</span>
        {/if}
      </a>
    {/each}
  </nav>

  <div class="border-t border-[var(--border)] p-2">
    <button
      type="button"
      onclick={() => (collapsed = !collapsed)}
      class="flex w-full items-center justify-center rounded-md p-2 text-[var(--muted-foreground)] transition-colors hover:bg-[var(--muted)] hover:text-[var(--foreground)]"
      aria-label={collapsed ? 'Expandir sidebar' : 'Colapsar sidebar'}
    >
      {#if collapsed}
        <ChevronsRight class="h-4 w-4" aria-hidden="true" />
      {:else}
        <ChevronsLeft class="h-4 w-4" aria-hidden="true" />
      {/if}
    </button>
  </div>
</aside>
