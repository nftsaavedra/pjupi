use tauri::State;
use crate::domain::estadisticas::{DocenteProyectosCount, KpisDashboard, ExportData};
use crate::domain::proyecto::ExportDataConProjectos;
use crate::error::AppError;
use crate::state::AppState;
use crate::storage;

#[tauri::command]
pub async fn get_estadisticas_proyectos_x_docente(
    state: State<'_, AppState>,
    actor_user_id: String,
) -> Result<Vec<DocenteProyectosCount>, AppError> {
    storage::get_estadisticas_proyectos_x_docente(&state, &actor_user_id).await
}

#[tauri::command]
pub async fn get_kpis_dashboard(
    state: State<'_, AppState>,
    actor_user_id: String,
) -> Result<KpisDashboard, AppError> {
    storage::get_kpis_dashboard(&state, &actor_user_id).await
}

#[tauri::command]
pub async fn get_data_exportacion_plana(
    state: State<'_, AppState>,
    actor_user_id: String,
) -> Result<Vec<ExportData>, AppError> {
    storage::get_data_exportacion_plana(&state, &actor_user_id).await
}

// NEW: Improved export grouped by docente
#[tauri::command]
pub async fn get_data_exportacion_agrupada_docente(
    state: State<'_, AppState>,
    actor_user_id: String,
) -> Result<Vec<ExportDataConProjectos>, AppError> {
    storage::get_data_exportacion_agrupada_docente(&state, &actor_user_id).await
}