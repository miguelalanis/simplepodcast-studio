import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
  plugins: [svelte(), tailwindcss()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: 'localhost',
    hmr: {
      protocol: 'ws',
      host: 'localhost',
      port: 1421,
    },
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
});
