/**
 * Store global de routing basado en hash.
 * Svelte 5 runes — equivalente a svelte-spa-router pero sin la dep.
 *
 * Rutas de la app:
 *   /            → Dashboard
 *   /editor      → Editor (nuevo episodio)
 *   /editor/:id  → Editor (edición)
 *   /settings    → Settings
 *   *            → NotFound
 */

import { onMount } from 'svelte';

const path = $state({ current: '/' });

let initialized = false;

export function initRouter() {
  if (initialized || typeof window === 'undefined') return;
  initialized = true;

  const update = () => {
    const hash = window.location.hash.slice(1) || '/';
    path.current = hash;
  };

  window.addEventListener('hashchange', update);
  update();
}

export function navigate(href: string) {
  window.location.hash = href;
}

export function getPath(): string {
  return path.current;
}

export function getParams(path: string, pattern: string): Record<string, string> {
  const patternParts = pattern.split('/').filter(Boolean);
  const pathParts = path.split('/').filter(Boolean);
  if (patternParts.length !== pathParts.length) return {};
  const params: Record<string, string> = {};
  for (let i = 0; i < patternParts.length; i++) {
    const p = patternParts[i].replace(':', '');
    if (!p) return {};
    params[p] = decodeURIComponent(pathParts[i]);
  }
  return params;
}

// Hook para usar en componentes y mantenerlos reactivos al path.
export function usePath(): string {
  // Acceder a path.current crea la dependencia reactiva.
  // eslint-disable-next-line @typescript-eslint/no-unused-expressions
  path.current;
  return path.current;
}

// Compat: para usar con Svelte 5, onMount garantiza init.
export function ensureRouter() {
  onMount(() => initRouter());
}
