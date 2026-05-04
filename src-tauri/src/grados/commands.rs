use tauri::{State, Window};
use crate::grados::models::{GradoAcademico, CreateGradoRequest, EliminarGradoResultado};
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use crate::shared::access_control;

#[tauri::command]
pub async fn get_all_grados(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Vec<GradoAcademico>, AppError> {
    access_control::get_all_grados(&state, window.label()).await
}

#[tauri::command]
pub async fn crear_grado(
    window: Window,
    state: State<'_, AppState>,
    request: CreateGradoRequest,
) -> Result<GradoAcademico, AppError> {
    access_control::crear_grado(&state, window.label(), request).await
}

#[tauri::command]
pub async fn actualizar_grado(
    window: Window,
    state: State<'_, AppState>,
    id_grado: String,
    request: CreateGradoRequest,
) -> Result<GradoAcademico, AppError> {
    access_control::actualizar_grado(&state, window.label(), &id_grado, request).await
}

#[tauri::command]
pub async fn eliminar_grado(
    window: Window,
    state: State<'_, AppState>,
    id_grado: String,
) -> Result<EliminarGradoResultado, AppError> {
    access_control::eliminar_grado(&state, window.label(), &id_grado).await
}

#[tauri::command]
pub async fn reactivar_grado(
    window: Window,
    state: State<'_, AppState>,
    id_grado: String,
) -> Result<GradoAcademico, AppError> {
    access_control::reactivar_grado(&state, window.label(), &id_grado).await
}
