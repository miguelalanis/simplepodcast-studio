/**
 * Wrapper minimalista sobre svelte-i18n v4 para uso en Svelte 5.
 *
 * svelte-i18n v4 cambió su API a stores y se vuelve complejo de tipar en runes.
 * Como MVP solo necesitamos un `t()` simple que devuelva la traducción del
 * diccionario activo. En Fase 2 (Fase 1.5.4) se reemplaza por algo más
 * completo con cambio de idioma en runtime.
 */

import es from './es.json';
import en from './en.json';

type Locale = 'es' | 'en';

const dictionaries: Record<Locale, Record<string, unknown>> = { es, en };

let currentLocale: Locale = 'es';

export function setLocale(locale: Locale) {
  currentLocale = locale;
  document.documentElement.lang = locale;
}

export function getLocale(): Locale {
  return currentLocale;
}

/**
 * Traduce una clave. Soporta interpolación básica con `{name}` en el valor.
 *
 * Ejemplo: t('dashboard.no_episodes', { action: 'Nuevo episodio' })
 */
export function t(key: string, vars?: Record<string, string | number>): string {
  const dict = dictionaries[currentLocale] ?? dictionaries.es;
  const value = dict[key];
  if (typeof value !== 'string') {
    console.warn(`[i18n] Missing translation: ${key}`);
    return key;
  }
  if (!vars) return value;
  return value.replace(/\{(\w+)\}/g, (_, name) => {
    const v = vars[name];
    return v === undefined ? `{${name}}` : String(v);
  });
}
