<script lang="ts">
  import { navigate } from '../lib/stores/router.svelte';
  import Button from '$lib/components/ui/Button.svelte';
</script>

<div class="mx-auto max-w-md text-center">
  <p class="text-5xl font-bold text-[var(--foreground)]">404</p>
  <h1 class="mt-4 text-xl font-semibold text-[var(--foreground)]">Página no encontrada</h1>
  <p class="mt-2 text-sm text-[var(--muted-foreground)]">
    La ruta que buscás no existe.
  </p>
  <Button class="mt-6" onclick={() => navigate('/')}>
    Volver al inicio
  </Button>
</div>
