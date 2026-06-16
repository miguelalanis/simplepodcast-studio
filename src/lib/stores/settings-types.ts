/**
 * Types compartidos entre Rust y TypeScript.
 *
 * Mantener en sync manualmente con src-tauri/src/config.rs.
 * Ver `project-specs.md` sección 8 para el patrón de duplicación.
 */

export type Theme = 'light' | 'dark' | 'system';
export type Language = 'es' | 'en';
export type AuthMethod = 'device_flow' | 'pat';
