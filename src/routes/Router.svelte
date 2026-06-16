<script lang="ts">
  import type { Component } from 'svelte';
  import Dashboard from './Dashboard.svelte';
  import Editor from './Editor.svelte';
  import Settings from './Settings.svelte';
  import NotFound from './NotFound.svelte';
  import { getParams, getPath, ensureRouter } from '../lib/stores/router.svelte';

  interface Props {
    children?: never;
  }
  const {}: Props = $props();

  ensureRouter();

  const routes: { pattern: string; component: Component }[] = [
    { pattern: '/', component: Dashboard },
    { pattern: '/editor', component: Editor },
    { pattern: '/editor/:id', component: Editor },
    { pattern: '/settings', component: Settings },
  ];

  const current = $derived.by(() => {
    const p = getPath();
    for (const r of routes) {
      if (r.pattern === p) return { component: r.component, params: {} };
      if (r.pattern.includes(':')) {
        const params = getParams(p, r.pattern);
        if (Object.keys(params).length) return { component: r.component, params };
      }
    }
    return { component: NotFound, params: {} };
  });
</script>

<svelte:component this={current.component} params={current.params} />
