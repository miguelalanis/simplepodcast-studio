# simplepodcast-studio

Editor de escritorio local-first para publicar podcasts. Anti-lock-in.

> El usuario es dueño de su repo (plantilla forkable) y puede dejar de usar Studio
> en cualquier momento sin perder nada. Los archivos son markdown y JSON estándar.

## Stack

- **Tauri 2** (Rust + WebView) para la app de escritorio.
- **Svelte 5** (runes) + **Vite 6** para el frontend.
- **shadcn-svelte** (button, input, card) para la UI base.
- **Router hash-based** casero (reemplazo a svelte-spa-router v4, incompatible con Svelte 5 runes).
- **i18n** wrapper minimalista (`t()` función pura, sin store).
- **Tailwind v4** con `@tailwindcss/vite` y `@tailwindcss/typography`.

## Requisitos

- Node.js >= 22.12
- pnpm >= 11
- Rust stable (para `tauri dev` y `tauri build`).

## Setup

```sh
pnpm install
pnpm tauri dev
```

La primera vez, `cargo check` descarga y compila las dependencias de Rust (~5-10 min).

## Build

```sh
pnpm tauri build
```

Genera los instaladores en `src-tauri/target/release/bundle/`.

## Auto-update (GitHub Releases)

Los updates se sirven desde GitHub Releases vía `tauri-plugin-updater`.

### Setup de signing (una sola vez)

1. Generar keypair Ed25519:
   ```sh
   pnpm tauri signer generate -w ~/.tauri/simplepodcast-studio.key -p ""
   ```
2. Copiar la clave privada (`~/.tauri/simplepodcast-studio.key`) y guardarla como secret en GitHub:
   - Repo → Settings → Secrets and variables → Actions → New repository secret
   - Name: `TAURI_SIGNING_PRIVATE_KEY`
   - Value: contenido del archivo `.key`
3. El workflow `.github/workflows/release.yml` usa `tauri-apps/tauri-action@v0` para:
   - Buildar binarios para macOS (aarch64 + x86_64), Windows (x64) y Linux (x64).
   - Firmarlos con la clave privada.
   - Crear un GitHub Release draft con los binarios y `latest.json`.
   - El updater endpoint apunta a `https://github.com/miguelalanis/simplepodcast-studio/releases/latest/download/latest.json`.

### Crear un release

```sh
git tag v0.1.0
git push origin v0.1.0
```

GitHub Actions buildea, firma, y publica el release. Los usuarios con la app instalada reciben el update automáticamente.

## Estructura

```
src/                 # Frontend Svelte 5
├── App.svelte
├── main.ts
├── app.css          # shadcn CSS variables + tema
├── lib/
│   ├── i18n/        # wrapper minimalista de i18n
│   ├── stores/      # router, settings, types
│   ├── components/  # ui/ (shadcn), layout/ (header/sidebar), common/
│   └── utils/       # cn() helper
└── routes/          # Router hash-based (Dashboard, Editor, Settings, NotFound)

src-tauri/           # Backend Rust
├── src/
│   ├── main.rs
│   ├── lib.rs       # Builder + plugins + commands
│   ├── config.rs    # AppConfig (zod-mirror)
│   ├── error.rs     # AppError con user_message() en español
│   └── commands/    # Tauri commands (settings, open_url, etc.)
└── capabilities/    # ACL Tauri 2 (default, fs, shell, net, store, dialog)
```

## Licencia

BSL 1.1 → Apache 2.0 en 2030-06-15. Ver `LICENSE`.

## Repos relacionados

- [`simplepodcast-template`](https://github.com/miguelalanis/simplepodcast-template) — plantilla Astro SSG.
