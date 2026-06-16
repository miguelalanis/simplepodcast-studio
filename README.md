# simplepodcast-studio

Editor de escritorio local-first para publicar podcasts. Anti-lock-in.

> El usuario es dueño de su repo (plantilla forkable) y puede dejar de usar Studio
> en cualquier momento sin perder nada. Los archivos son markdown y JSON estándar.

## Stack

- **Tauri 2** (Rust + WebView) para la app de escritorio.
- **Svelte 5** (runes) + **Vite** para el frontend.
- **shadcn-svelte** para la UI base (no incluido en este scaffold).
- **svelte-spa-router** para el routing SPA.
- **svelte-i18n** para mensajes (solo español completo en MVP).
- **tailwindcss v4** (Tailwind v4 con `@tailwindcss/vite`).

## Requisitos

- Node.js >= 22.12
- pnpm >= 11
- Rust stable (para `tauri dev` y `tauri build`).

## Setup

```sh
pnpm install
pnpm tauri dev
```

## Build

```sh
pnpm tauri build
```

Genera los instaladores en `src-tauri/target/release/bundle/`.

## Estructura

```
src/                 # Frontend Svelte 5
├── App.svelte
├── main.ts
├── app.css
├── lib/
│   ├── i18n/        # svelte-i18n
│   ├── components/   # layout/, ui/, common/
│   └── utils/
└── routes/           # páginas SPA (svelte-spa-router)

src-tauri/           # Backend Rust
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config.rs     # AppConfig (zod-mirror)
│   ├── error.rs      # AppError
│   └── commands/     # Tauri commands
└── capabilities/      # ACL Tauri 2
```

## Licencia

BSL 1.1 → Apache 2.0 en 2030-06-15. Ver `LICENSE`.

## Repos relacionados

- [`simplepodcast-template`](https://github.com/miguelalanis/simplepodcast-template) — plantilla Astro SSG.
