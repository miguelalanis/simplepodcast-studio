<script lang="ts">
  import { Save, Eye } from '@lucide/svelte';
  import { t } from '../lib/i18n';
  import { getParams, getPath, ensureRouter } from '../lib/stores/router.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Card from '$lib/components/ui/Card.svelte';

  ensureRouter();

  const params = $derived.by(() => {
    const p = getPath();
    return getParams(p, '/editor/:id');
  });

  const isEditing = $derived(!!params.id);
</script>

<div class="flex h-full gap-6">
  <!-- Sidebar de metadatos -->
  <Card class="w-72 flex-shrink-0 overflow-y-auto p-5 space-y-4">
    <div class="flex items-center justify-between">
      <h2 class="text-sm font-semibold uppercase tracking-wider text-[var(--muted-foreground)]">
        Metadatos
      </h2>
      {#if isEditing}
        <span class="rounded bg-[var(--muted)] px-2 py-0.5 font-mono text-xs text-[var(--muted-foreground)]">
          {params.id}
        </span>
      {/if}
    </div>

    <label class="block">
      <span class="text-xs font-medium text-[var(--foreground)]">{t('editor.fields.title')}</span>
      <Input type="text" placeholder="Título del episodio" class="mt-1" />
    </label>

    <label class="block">
      <span class="text-xs font-medium text-[var(--foreground)]">{t('editor.fields.slug')}</span>
      <Input type="text" placeholder="titulo-del-episodio" class="mt-1 font-mono" />
    </label>

    <label class="block">
      <span class="text-xs font-medium text-[var(--foreground)]">{t('editor.fields.pub_date')}</span>
      <Input type="date" class="mt-1" />
    </label>

    <label class="block">
      <span class="text-xs font-medium text-[var(--foreground)]">{t('editor.fields.status')}</span>
      <select
        class="mt-1 flex h-9 w-full rounded-md border border-[var(--border)] bg-transparent px-3 py-1 text-sm shadow-sm focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-[var(--ring)]"
      >
        <option value="draft">Borrador</option>
        <option value="published">Publicado</option>
        <option value="scheduled">Programado</option>
      </select>
    </label>

    <label class="block">
      <span class="text-xs font-medium text-[var(--foreground)]">{t('editor.fields.short_description')}</span>
      <textarea
        rows="3"
        placeholder="Descripción corta para SEO y RSS."
        class="mt-1 flex w-full rounded-md border border-[var(--border)] bg-transparent px-3 py-2 text-sm shadow-sm placeholder:text-[var(--muted-foreground)] focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-[var(--ring)]"
      ></textarea>
    </label>

    <div class="grid grid-cols-2 gap-2">
      <label class="block">
        <span class="text-xs font-medium text-[var(--foreground)]">{t('editor.fields.explicit')}</span>
        <select
          class="mt-1 flex h-9 w-full rounded-md border border-[var(--border)] bg-transparent px-3 py-1 text-sm shadow-sm focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-[var(--ring)]"
        >
          <option value="false">No</option>
          <option value="true">Sí</option>
        </select>
      </label>
      <label class="block">
        <span class="text-xs font-medium text-[var(--foreground)]">Duración</span>
        <Input type="text" placeholder="00:11:30" class="mt-1 font-mono" />
      </label>
    </div>

    <div class="flex gap-2 pt-2">
      <Button variant="outline" size="sm" class="flex-1">
        <Eye class="h-3.5 w-3.5" />
        {t('editor.preview')}
      </Button>
      <Button size="sm" class="flex-1">
        <Save class="h-3.5 w-3.5" />
        {t('common.save')}
      </Button>
    </div>
  </Card>

  <!-- Área de contenido (Tiptap placeholder) -->
  <div class="flex flex-1 flex-col gap-4">
    <!-- Toolbar placeholder -->
    <Card class="flex items-center gap-1 p-2">
      <span class="text-xs text-[var(--muted-foreground)]">
        Toolbar de Tiptap (T1.5.6) — bold, italic, headings, listas, link, imagen, undo/redo.
      </span>
    </Card>

    <!-- Editor placeholder -->
    <Card class="flex-1 overflow-y-auto p-6">
      <div class="prose max-w-none">
        <h1 class="text-3xl font-bold text-[var(--foreground)]">
          {isEditing ? `Editando: ${params.id}` : 'Nuevo episodio'}
        </h1>
        <p class="text-[var(--muted-foreground)]">
          Este es el placeholder del editor de contenido con Tiptap.
          En T1.5.6 se implementa el wrapper de Tiptap con `tiptap-markdown`
          para WYSIWYG bidireccional HTML ↔ markdown.
        </p>
        <h2 class="text-xl font-semibold text-[var(--foreground)]">Contenido de ejemplo</h2>
        <p>
          El editor soportará headings, bold, italic, listas, blockquotes,
          code blocks, links, imágenes, y más. Todo se serializa a markdown
          estándar para mantener la compatibilidad con cualquier herramienta.
        </p>
        <blockquote class="border-l-4 border-[var(--primary)] pl-4 text-[var(--muted-foreground)]">
          Si dejás de usar Studio, tus archivos siguen siendo markdown legible.
          Anti-lock-in por diseño.
        </blockquote>
      </div>
    </Card>

    <!-- Barra de acciones inferior -->
    <Card class="flex items-center justify-between p-3">
      <span class="text-xs text-[var(--muted-foreground)]">
        {isEditing ? `Editando episodio ${params.id}` : 'Episodio nuevo — los cambios se guardan como borrador.'}
      </span>
      <div class="flex gap-2">
        <Button variant="ghost" size="sm">
          <Eye class="h-3.5 w-3.5" />
          {t('editor.preview')}
        </Button>
        <Button size="sm">
          <Save class="h-3.5 w-3.5" />
          {t('common.save')}
        </Button>
      </div>
    </Card>
  </div>
</div>
