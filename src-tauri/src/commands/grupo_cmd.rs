use tauri::{State, Window};
use crate::domain::grupo_investigacion::{GrupoInvestigacion, CreateGrupoInvestigacionRequest, UpdateGrupoInvestigacionRequest};
use crate::error::AppError;
use crate::state::AppState;
use crate::storage;

#[tauri::command]
pub async fn get_all_grupos(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Vec<GrupoInvestigacion>, AppError> {
    storage::get_all_grupos(&state, window.label()).await
}

#[tauri::command]
pub async fn create_grupo(
    window: Window,
    state: State<'_, AppState>,
    request: CreateGrupoInvestigacionRequest,
) -> Result<GrupoInvestigacion, AppError> {
    storage::create_grupo(&state, window.label(), request).await
}

#[tauri::command]
pub async fn get_grupo(
    window: Window,
    state: State<'_, AppState>,
    id_grupo: String,
) -> Result<GrupoInvestigacion, AppError> {
    storage::get_grupo(&state, window.label(), &id_grupo).await
}

#[tauri::command]
pub async fn update_grupo(
    window: Window,
    state: State<'_, AppState>,
    id_grupo: String,
    request: UpdateGrupoInvestigacionRequest,
) -> Result<GrupoInvestigacion, AppError> {
    storage::update_grupo(&state, window.label(), &id_grupo, request).await
}

#[tauri::command]
pub async fn delete_grupo(
    window: Window,
    state: State<'_, AppState>,
    id_grupo: String,
) -> Result<(), AppError> {
    storage::delete_grupo(&state, window.label(), &id_grupo).await
}
