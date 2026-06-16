//! Comando para abrir URLs en el navegador default del SO.
//!
//! Usado por:
//! - El botón "Vista previa" del editor de episodios y config (Nivel 0-2).
//! - El botón "Conectar con GitHub" durante el Device Flow (Nivel 1+).
//! - El link "Ver sitio en producción" después del deploy (Nivel 2).

use tauri_plugin_opener::OpenerExt;

use crate::error::AppError;

#[tauri::command]
pub async fn open_url(app: tauri::AppHandle, url: String) -> Result<(), AppError> {
    if url.is_empty() {
        return Err(AppError::Other {
            message: "URL vacía".to_string(),
        });
    }
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|e| AppError::Other { message: e.to_string() })?;
    Ok(())
}
