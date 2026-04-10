use tauri::{State, Window};
use crate::domain::grado::{GradoAcademico, CreateGradoRequest, EliminarGradoResultado};
use crate::error::AppError;
use crate::state::AppState;
use crate::storage;

#[tauri::command]
pub async fn get_all_grados(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Vec<GradoAcademico>, AppError> {
    storage::get_all_grados(&state, window.label()).await
}

#[tauri::command]
pub async fn crear_grado(
    window: Window,
    state: State<'_, AppState>,
    request: CreateGradoRequest,
) -> Result<GradoAcademico, AppError> {
    storage::crear_grado(&state, window.label(), request).await
}

#[tauri::command]
pub async fn actualizar_grado(
    window: Window,
    state: State<'_, AppState>,
    id_grado: String,
    request: CreateGradoRequest,
) -> Result<GradoAcademico, AppError> {
    storage::actualizar_grado(&state, window.label(), &id_grado, request).await
}

#[tauri::command]
pub async fn eliminar_grado(
    window: Window,
    state: State<'_, AppState>,
    id_grado: String,
) -> Result<EliminarGradoResultado, AppError> {
    storage::eliminar_grado(&state, window.label(), &id_grado).await
}

#[tauri::command]
pub async fn reactivar_grado(
    window: Window,
    state: State<'_, AppState>,
    id_grado: String,
) -> Result<GradoAcademico, AppError> {
    storage::reactivar_grado(&state, window.label(), &id_grado).await
}