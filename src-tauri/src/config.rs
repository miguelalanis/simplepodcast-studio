//! Estructuras de configuración persistente.
//!
//! Se almacenan en `~/.simplepodcast/config.json` (preferencias no sensibles)
//! y en el keychain del SO (secretos como tokens).

use serde::{Deserialize, Serialize};

/// Versión del schema. Incrementar cuando se hagan breaking changes.
pub const CONFIG_VERSION: &str = "1";

/// Configuración completa de la app, persistida en `~/.simplepodcast/config.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: String,
    #[serde(default)]
    pub github: GitHubConfig,
    #[serde(default)]
    pub cloudflare: CloudflareConfig,
    #[serde(default)]
    pub r2: R2Config,
    pub preferences: Preferences,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: CONFIG_VERSION.to_string(),
            github: GitHubConfig::default(),
            cloudflare: CloudflareConfig::default(),
            r2: R2Config::default(),
            preferences: Preferences::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    pub auth_method: AuthMethod,
    pub username: String,
}

impl Default for GitHubConfig {
    fn default() -> Self {
        Self {
            auth_method: AuthMethod::DeviceFlow,
            username: String::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuthMethod {
    DeviceFlow,
    Pat,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloudflareConfig {
    pub account_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct R2Config {
    pub account_id: String,
    pub bucket: String,
    pub public_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    /// Carpeta raíz para clonar repos. Por defecto `~/SimplePodcast`.
    pub clone_root: String,
    pub theme: Theme,
    pub language: Language,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            clone_root: String::new(),
            theme: Theme::System,
            language: Language::Es,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Es,
    En,
}
