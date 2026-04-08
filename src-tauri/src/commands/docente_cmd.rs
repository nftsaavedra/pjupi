use tauri::State;
use crate::domain::docente::{CreateDocenteRequest, Docente, DocenteDetalle, EliminarDocenteResultado};
use crate::error::AppError;
use crate::state::AppState;
use crate::storage;

#[tauri::command]
pub async fn crear_docente(
    state: State<'_, AppState>,
    request: CreateDocenteRequest,
) -> Result<Docente, AppError> {
    storage::crear_docente(&state, request).await
}

#[tauri::command]
pub async fn get_all_docentes(
    state: State<'_, AppState>,
) -> Result<Vec<Docente>, AppError> {
    storage::get_all_docentes(&state).await
}

// NEW: Get docentes with project details
#[tauri::command]
pub async fn get_all_docentes_con_proyectos(
    state: State<'_, AppState>,
) -> Result<Vec<DocenteDetalle>, AppError> {
    storage::get_all_docentes_con_proyectos(&state).await
}

#[tauri::command]
pub async fn eliminar_docente(
    state: State<'_, AppState>,
    id_docente: String,
) -> Result<EliminarDocenteResultado, AppError> {
    storage::eliminar_docente(&state, &id_docente).await
}

#[tauri::command]
pub async fn reactivar_docente(
    state: State<'_, AppState>,
    id_docente: String,
) -> Result<Docente, AppError> {
    storage::reactivar_docente(&state, &id_docente).await
}