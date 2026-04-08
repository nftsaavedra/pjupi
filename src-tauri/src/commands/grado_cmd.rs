use tauri::State;
use crate::domain::grado::{GradoAcademico, CreateGradoRequest, EliminarGradoResultado};
use crate::error::AppError;
use crate::state::AppState;
use crate::storage;

#[tauri::command]
pub async fn get_all_grados(
    state: State<'_, AppState>,
) -> Result<Vec<GradoAcademico>, AppError> {
    storage::get_all_grados(&state).await
}

#[tauri::command]
pub async fn crear_grado(
    state: State<'_, AppState>,
    request: CreateGradoRequest,
) -> Result<GradoAcademico, AppError> {
    storage::crear_grado(&state, request).await
}

#[tauri::command]
pub async fn actualizar_grado(
    state: State<'_, AppState>,
    id_grado: String,
    request: CreateGradoRequest,
) -> Result<GradoAcademico, AppError> {
    storage::actualizar_grado(&state, &id_grado, request).await
}

#[tauri::command]
pub async fn eliminar_grado(
    state: State<'_, AppState>,
    id_grado: String,
) -> Result<EliminarGradoResultado, AppError> {
    storage::eliminar_grado(&state, &id_grado).await
}

#[tauri::command]
pub async fn reactivar_grado(
    state: State<'_, AppState>,
    id_grado: String,
) -> Result<GradoAcademico, AppError> {
    storage::reactivar_grado(&state, &id_grado).await
}