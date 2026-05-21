use tauri::{State, Window};

use crate::docentes::models::{Publicacion, SyncPublicacionesResult};
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use crate::shared::access_control;
use crate::shared::external::pure_service;

#[tauri::command]
pub async fn sincronizar_publicaciones_pure(
    window: Window,
    state: State<'_, AppState>,
    docente_id: String,
) -> Result<SyncPublicacionesResult, AppError> {
    access_control::require_docentes_manage_permission(&state, window.label()).await?;
    pure_service::sync_publicaciones(&state, &docente_id).await
}

#[tauri::command]
pub async fn get_publicaciones_docente(
    window: Window,
    state: State<'_, AppState>,
    docente_id: String,
) -> Result<Vec<Publicacion>, AppError> {
    access_control::require_docentes_view_permission(&state, window.label()).await?;
    pure_service::get_publicaciones(&state, &docente_id).await
}
