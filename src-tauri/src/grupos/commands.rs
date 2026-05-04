use tauri::{State, Window};
use crate::grupos::models::{GrupoInvestigacion, CreateGrupoInvestigacionRequest, UpdateGrupoInvestigacionRequest};
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use crate::shared::access_control;

#[tauri::command]
pub async fn get_all_grupos(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Vec<GrupoInvestigacion>, AppError> {
    access_control::get_all_grupos(&state, window.label()).await
}

#[tauri::command]
pub async fn create_grupo(
    window: Window,
    state: State<'_, AppState>,
    request: CreateGrupoInvestigacionRequest,
) -> Result<GrupoInvestigacion, AppError> {
    access_control::create_grupo(&state, window.label(), request).await
}

#[tauri::command]
pub async fn get_grupo(
    window: Window,
    state: State<'_, AppState>,
    id_grupo: String,
) -> Result<GrupoInvestigacion, AppError> {
    access_control::get_grupo(&state, window.label(), &id_grupo).await
}

#[tauri::command]
pub async fn update_grupo(
    window: Window,
    state: State<'_, AppState>,
    id_grupo: String,
    request: UpdateGrupoInvestigacionRequest,
) -> Result<GrupoInvestigacion, AppError> {
    access_control::update_grupo(&state, window.label(), &id_grupo, request).await
}

#[tauri::command]
pub async fn delete_grupo(
    window: Window,
    state: State<'_, AppState>,
    id_grupo: String,
) -> Result<(), AppError> {
    access_control::delete_grupo(&state, window.label(), &id_grupo).await
}
