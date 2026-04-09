use tauri::State;
use crate::domain::docente::{CreateDocenteRequest, Docente, DocenteDetalle, EliminarDocenteResultado, ReniecDniLookupResult};
use crate::error::AppError;
use crate::infrastructure::reniec_client;
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

#[tauri::command]
pub async fn buscar_docente_por_dni(
    state: State<'_, AppState>,
    dni: String,
) -> Result<Option<Docente>, AppError> {
    storage::buscar_docente_por_dni(&state, &dni).await
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

#[tauri::command]
pub async fn consultar_dni_reniec(
    state: State<'_, AppState>,
    numero: String,
) -> Result<ReniecDniLookupResult, AppError> {
    reniec_client::consultar_dni(state.reniec_config(), &numero).await
}