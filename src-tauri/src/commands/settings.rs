//! Comandos de settings (persistencia de preferencias + gestión de credenciales).
//!
//! T1.4 del roadmap. Las preferencias no sensibles viven en
//! `~/.simplepodcast/config.json`. Los tokens viven en el keychain del SO.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use tauri::Manager;

use crate::config::{AppConfig, AuthMethod, CONFIG_VERSION};
use crate::error::AppError;

const CONFIG_DIR: &str = "SimplePodcast";
const CONFIG_FILE: &str = "config.json";

/// Devuelve la ruta a `~/.simplepodcast/config.json`.
fn config_path(app: &tauri::AppHandle) -> Result<PathBuf, AppError> {
    let home = app
        .path()
        .home_dir()
        .map_err(|e| AppError::Config { message: e.to_string() })?;
    Ok(home.join(CONFIG_DIR).join(CONFIG_FILE))
}

/// Lee el config del usuario. Si no existe, devuelve un AppConfig con defaults.
#[tauri::command]
pub async fn settings_get(app: tauri::AppHandle) -> Result<AppConfig, AppError> {
    let path = config_path(&app)?;
    if !path.exists() {
        return Ok(AppConfig::default());
    }
    let raw = fs::read_to_string(&path).map_err(|e| AppError::Config {
        message: format!("No pude leer {}: {}", path.display(), e),
    })?;
    let cfg: AppConfig = serde_json::from_str(&raw).map_err(|e| AppError::Config {
        message: format!("JSON inválido en {}: {}", path.display(), e),
    })?;
    Ok(cfg)
}

/// Escribe el config atómicamente: write-to-temp + rename.
#[tauri::command]
pub async fn settings_set(
    app: tauri::AppHandle,
    config: AppConfig,
) -> Result<(), AppError> {
    let path = config_path(&app)?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| AppError::Config {
            message: format!("No pude crear {}: {}", parent.display(), e),
        })?;
    }

    // Validar versión antes de escribir.
    if config.version != CONFIG_VERSION {
        return Err(AppError::Config {
            message: format!(
                "Versión de config no soportada: {} (esperado {})",
                config.version, CONFIG_VERSION
            ),
        });
    }

    let json = serde_json::to_string_pretty(&config).map_err(|e| AppError::Config {
        message: format!("No pude serializar config: {}", e),
    })?;

    // Escritura atómica.
    let tmp = path.with_extension("json.tmp");
    {
        let mut f = fs::File::create(&tmp).map_err(|e| AppError::FileWrite {
            path: tmp.display().to_string(),
        })?;
        f.write_all(json.as_bytes()).map_err(|e| AppError::FileWrite {
            path: tmp.display().to_string(),
        })?;
        f.sync_all().map_err(|e| AppError::FileWrite {
            path: tmp.display().to_string(),
        })?;
    }
    fs::rename(&tmp, &path).map_err(|e| AppError::Config {
        message: format!("No pude renombrar {}: {}", tmp.display(), e),
    })?;

    tracing::info!(path = %path.display(), "config guardado");
    Ok(())
}

/// Borra credenciales sensibles (tokens de GitHub/Cloudflare/R2) del keychain.
#[tauri::command]
pub async fn settings_clear_credentials() -> Result<(), AppError> {
    let keyring_id = "simplepodcast-studio";

    let entries = [
        ("github-token", AuthMethod::DeviceFlow),
        ("github-token", AuthMethod::Pat),
        ("cloudflare-api-token", AuthMethod::DeviceFlow),
        ("r2-access-key", AuthMethod::DeviceFlow),
        ("r2-secret-key", AuthMethod::DeviceFlow),
    ];

    for (key, _method) in entries {
        match keyring::Entry::new(keyring_id, key) {
            Ok(entry) => match entry.delete_credential() {
                Ok(()) => tracing::info!(key, "credencial borrada"),
                Err(keyring::Error::NoEntry) => {} // no existe, no-op
                Err(e) => tracing::warn!(key, error = %e, "error al borrar credencial"),
            },
            Err(e) => tracing::warn!(key, error = %e, "error al acceder al keyring"),
        }
    }

    Ok(())
}

/// Lee una credencial del keychain (helper interno).
pub fn read_credential(key: &str) -> Result<String, AppError> {
    let entry = keyring::Entry::new("simplepodcast-studio", key)
        .map_err(|e| AppError::Config { message: e.to_string() })?;
    entry.get_password().map_err(|e| match e {
        keyring::Error::NoEntry => AppError::Config {
            message: format!("No hay credencial guardada para {}", key),
        },
        _ => AppError::Config { message: e.to_string() },
    })
}

/// Escribe una credencial en el keychain (helper interno, no es command).
pub fn write_credential(key: &str, value: &str) -> Result<(), AppError> {
    let entry = keyring::Entry::new("simplepodcast-studio", key)
        .map_err(|e| AppError::Config { message: e.to_string() })?;
    entry
        .set_password(value)
        .map_err(|e| AppError::Config { message: e.to_string() })?;
    Ok(())
}

/// Verifica la ruta del config (solo para debugging / smoke tests).
#[tauri::command]
pub async fn settings_path(app: tauri::AppHandle) -> Result<String, AppError> {
    Ok(config_path(&app)?.display().to_string())
}

// Evita warning de unused si en el futuro se referencian funciones helpers.
#[allow(dead_code)]
fn _force_link(_p: &Path) {}
