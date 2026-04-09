use tauri::State;
use crate::domain::docente::{CreateDocenteRequest, Docente, DocenteDetalle, EliminarDocenteResultado, RefreshDocenteRenacytFormacionResultado, RenacytLookupResult, ReniecDniLookupResult};
use crate::error::AppError;
use crate::infrastructure::renacyt_client;
use crate::infrastructure::reniec_client;
use crate::state::AppState;
use crate::storage;

#[tauri::command]
pub async fn crear_docente(
    state: State<'_, AppState>,
    actor_user_id: String,
    request: CreateDocenteRequest,
) -> Result<Docente, AppError> {
    storage::crear_docente(&state, &actor_user_id, request).await
}

#[tauri::command]
pub async fn get_all_docentes(
    state: State<'_, AppState>,
    actor_user_id: String,
) -> Result<Vec<Docente>, AppError> {
    storage::get_all_docentes(&state, &actor_user_id).await
}

#[tauri::command]
pub async fn buscar_docente_por_dni(
    state: State<'_, AppState>,
    actor_user_id: String,
    dni: String,
) -> Result<Option<Docente>, AppError> {
    storage::buscar_docente_por_dni(&state, &actor_user_id, &dni).await
}

// NEW: Get docentes with project details
#[tauri::command]
pub async fn get_all_docentes_con_proyectos(
    state: State<'_, AppState>,
    actor_user_id: String,
) -> Result<Vec<DocenteDetalle>, AppError> {
    storage::get_all_docentes_con_proyectos(&state, &actor_user_id).await
}

#[tauri::command]
pub async fn eliminar_docente(
    state: State<'_, AppState>,
    actor_user_id: String,
    id_docente: String,
) -> Result<EliminarDocenteResultado, AppError> {
    storage::eliminar_docente(&state, &actor_user_id, &id_docente).await
}

#[tauri::command]
pub async fn reactivar_docente(
    state: State<'_, AppState>,
    actor_user_id: String,
    id_docente: String,
) -> Result<Docente, AppError> {
    storage::reactivar_docente(&state, &actor_user_id, &id_docente).await
}

#[tauri::command]
pub async fn consultar_dni_reniec(
    state: State<'_, AppState>,
    actor_user_id: String,
    numero: String,
) -> Result<ReniecDniLookupResult, AppError> {
    storage::require_docentes_manage_permission(&state, &actor_user_id).await?;
    reniec_client::consultar_dni(state.reniec_config(), &numero).await
}

#[tauri::command]
pub async fn consultar_renacyt_docente(
    state: State<'_, AppState>,
    actor_user_id: String,
    codigo_o_id: String,
) -> Result<RenacytLookupResult, AppError> {
    storage::require_docentes_manage_permission(&state, &actor_user_id).await?;
    renacyt_client::consultar_investigador(state.renacyt_config(), &codigo_o_id).await
}

#[tauri::command]
pub async fn refrescar_formacion_academica_renacyt_docente(
    state: State<'_, AppState>,
    actor_user_id: String,
    id_docente: String,
) -> Result<RefreshDocenteRenacytFormacionResultado, AppError> {
    storage::refrescar_formacion_academica_renacyt_docente(&state, &actor_user_id, &id_docente).await
}