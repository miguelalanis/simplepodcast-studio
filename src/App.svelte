<script lang="ts">
  import { onMount } from 'svelte';
  import { setLocale } from './lib/i18n';

  import AppHeader from './lib/components/layout/AppHeader.svelte';
  import Sidebar from './lib/components/layout/Sidebar.svelte';
  import Router from './routes/Router.svelte';

  let ready = $state(false);

  onMount(() => {
    // Idioma por default: español.
    setLocale('es');
    ready = true;
  });
</script>

<div class="flex h-full flex-col">
  <AppHeader />
  <div class="flex flex-1 overflow-hidden">
    <Sidebar />
    <main class="flex-1 overflow-y-auto bg-[var(--color-bg-subtle)] p-6">
      {#if ready}
        <Router />
      {/if}
    </main>
  </div>
</div>
