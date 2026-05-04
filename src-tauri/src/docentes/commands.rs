use tauri::{State, Window};
use crate::docentes::models::{CreateDocenteRequest, Docente, DocenteDetalle, EliminarDocenteResultado, RefreshDocenteRenacytFormacionResultado, RenacytLookupResult, ReniecDniLookupResult};
use crate::shared::error::AppError;
use crate::infrastructure::renacyt_client;
use crate::infrastructure::reniec_client;
use crate::shared::state::AppState;
use crate::shared::access_control;

#[tauri::command]
pub async fn crear_docente(
    window: Window,
    state: State<'_, AppState>,
    request: CreateDocenteRequest,
) -> Result<Docente, AppError> {
    access_control::crear_docente(&state, window.label(), request).await
}

#[tauri::command]
pub async fn get_all_docentes(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Vec<Docente>, AppError> {
    access_control::get_all_docentes(&state, window.label()).await
}

#[tauri::command]
pub async fn buscar_docente_por_dni(
    window: Window,
    state: State<'_, AppState>,
    dni: String,
) -> Result<Option<Docente>, AppError> {
    access_control::buscar_docente_por_dni(&state, window.label(), &dni).await
}

// NEW: Get docentes with project details
#[tauri::command]
pub async fn get_all_docentes_con_proyectos(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Vec<DocenteDetalle>, AppError> {
    access_control::get_all_docentes_con_proyectos(&state, window.label()).await
}

#[tauri::command]
pub async fn eliminar_docente(
    window: Window,
    state: State<'_, AppState>,
    id_docente: String,
) -> Result<EliminarDocenteResultado, AppError> {
    access_control::eliminar_docente(&state, window.label(), &id_docente).await
}

#[tauri::command]
pub async fn reactivar_docente(
    window: Window,
    state: State<'_, AppState>,
    id_docente: String,
) -> Result<Docente, AppError> {
    access_control::reactivar_docente(&state, window.label(), &id_docente).await
}

#[tauri::command]
pub async fn consultar_dni_reniec(
    window: Window,
    state: State<'_, AppState>,
    numero: String,
) -> Result<ReniecDniLookupResult, AppError> {
    access_control::require_docentes_manage_permission(&state, window.label()).await?;
    reniec_client::consultar_dni(state.reniec_config(), &numero).await
}

#[tauri::command]
pub async fn consultar_renacyt_docente(
    window: Window,
    state: State<'_, AppState>,
    codigo_o_id: String,
) -> Result<RenacytLookupResult, AppError> {
    access_control::require_docentes_manage_permission(&state, window.label()).await?;
    renacyt_client::consultar_investigador(state.renacyt_config(), &codigo_o_id).await
}

#[tauri::command]
pub async fn refrescar_formacion_academica_renacyt_docente(
    window: Window,
    state: State<'_, AppState>,
    id_docente: String,
) -> Result<RefreshDocenteRenacytFormacionResultado, AppError> {
    access_control::refrescar_formacion_academica_renacyt_docente(&state, window.label(), &id_docente).await
}
