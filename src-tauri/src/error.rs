//! Tipos de error de la app con mensajes amigables para el usuario.
//!
//! Patrón: cada variante tiene un mensaje técnico (en `Display`) y un
//! `user_message()` corto en español que es lo que se muestra en la UI.

use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("No se pudo conectar con {service}: {message}")]
    Network { service: String, message: String },

    #[error("Token inválido o expirado. Verificá que tenga los permisos correctos.")]
    InvalidToken,

    #[error("El repositorio no existe o no es accesible: {path}")]
    RepoNotFound { path: String },

    #[error("El archivo no se pudo escribir: {path}")]
    FileWrite { path: String },

    #[error("Git falló: {message}")]
    Git { message: String },

    #[error("Cloudflare error: {message}")]
    Cloudflare { message: String },

    #[error("R2 error: {message}")]
    R2 { message: String },

    #[error("Astro dev server: {message}")]
    Astro { message: String },

    #[error("La operación fue cancelada por el usuario")]
    Cancelled,

    #[error("Error de configuración: {message}")]
    Config { message: String },

    #[error("Error desconocido: {message}")]
    Other { message: String },
}

impl AppError {
    pub fn user_message(&self) -> String {
        match self {
            AppError::Network { service, .. } => {
                format!("No pudimos conectar con {}. Revisá tu conexión a internet.", service)
            }
            AppError::InvalidToken => {
                "Este token no funciona. Verificá que tenga los permisos correctos.".to_string()
            }
            AppError::RepoNotFound { path } => {
                format!("La carpeta {} no existe o no se puede leer.", path)
            }
            AppError::FileWrite { path } => {
                format!("No pudimos escribir en {}. Revisá los permisos.", path)
            }
            AppError::Git { .. } => {
                "Git falló. Revisá tu conexión o intentá de nuevo.".to_string()
            }
            AppError::Cloudflare { .. } => {
                "Cloudflare rechazó la operación. Verificá tu API token.".to_string()
            }
            AppError::R2 { .. } => {
                "No pudimos acceder al bucket. Verificá las credenciales.".to_string()
            }
            AppError::Astro { .. } => {
                "No pudimos iniciar el servidor. ¿Tenés Node.js instalado?".to_string()
            }
            AppError::Cancelled => "Operación cancelada.".to_string(),
            AppError::Config { message } => format!("Error de configuración: {}", message),
            AppError::Other { message } => format!("Algo salió mal: {}", message),
        }
    }
}

impl Serialize for AppError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.user_message())
    }
}
