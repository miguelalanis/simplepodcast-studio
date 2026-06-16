/**
 * Wrapper sobre los Tauri commands de settings (T1.4).
 *
 * La spec dice que `tauri-plugin-store` se usa para estado EFÍMERO (último
 * repo abierto, estado de dev server, preferencias de UI).
 *
 * Las preferencias persistentes (AppConfig: github username, cloudflare id, etc.)
 * viven en `~/.simplepodcast/config.json` via commands `settings_get/set`.
 *
 * Este módulo envuelve ambos. Cuando se llama desde el frontend:
 *   const cfg = await getSettings();
 *   await setSettings({...cfg, theme: 'dark'});
 *
 * Si no estamos en Tauri (e.g. `pnpm dev` standalone), devuelve defaults
 * para que la UI no rompa en desarrollo.
 */

import { invoke } from '@tauri-apps/api/core';

import type { Theme, Language } from './settings-types';

export interface AppConfig {
  version: string;
  github: {
    authMethod: 'device_flow' | 'pat';
    username: string;
  };
  cloudflare: {
    accountId: string;
  };
  r2: {
    accountId: string;
    bucket: string;
    publicUrl: string;
  };
  preferences: {
    cloneRoot: string;
    theme: Theme;
    language: Language;
  };
}

const DEFAULT_CONFIG: AppConfig = {
  version: '1',
  github: { authMethod: 'device_flow', username: '' },
  cloudflare: { accountId: '' },
  r2: { accountId: '', bucket: '', publicUrl: '' },
  preferences: { cloneRoot: '', theme: 'system', language: 'es' },
};

function isTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

export async function getSettings(): Promise<AppConfig> {
  if (!isTauri()) return DEFAULT_CONFIG;
  try {
    return await invoke<AppConfig>('settings_get');
  } catch (e) {
    console.warn('[settings] getSettings falló, usando defaults:', e);
    return DEFAULT_CONFIG;
  }
}

export async function setSettings(config: AppConfig): Promise<void> {
  if (!isTauri()) return;
  await invoke<void>('settings_set', { config });
}

export async function clearCredentials(): Promise<void> {
  if (!isTauri()) return;
  await invoke<void>('settings_clear_credentials');
}

export async function getSettingsPath(): Promise<string> {
  if (!isTauri()) return '(no Tauri)';
  return invoke<string>('settings_path');
}
