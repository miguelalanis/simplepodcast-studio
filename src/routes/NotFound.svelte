<script lang="ts">
  import { navigate } from '../lib/stores/router.svelte';
</script>

<div class="mx-auto max-w-md text-center">
  <p class="text-5xl font-bold text-[var(--color-text)]">404</p>
  <h1 class="mt-4 text-xl font-semibold text-[var(--color-text)]">Página no encontrada</h1>
  <p class="mt-2 text-sm text-[var(--color-text-muted)]">
    La ruta que buscás no existe.
  </p>
  <button
    type="button"
    onclick={() => navigate('/')}
    class="mt-6 inline-flex h-10 items-center rounded-md bg-[var(--color-accent-600)] px-4 text-sm font-medium text-white transition-colors hover:bg-[var(--color-accent-700)]"
  >
    Volver al inicio
  </button>
</div>
