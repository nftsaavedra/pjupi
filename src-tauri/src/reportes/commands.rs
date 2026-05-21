use tauri::{State, Window};
use crate::reportes::models::{DocenteProyectosCount, ExportData, ExportDataConProjectos, ExportDataDocentePerfil, ExportDataGrupo, ExportDataProyectoArea, ExportDataRecurso, KpisDashboard, ProyectosTrendItem, RenacytDistribucionItem};
use crate::reportes::entity_reports::{ReporteProyectoIntegral, ReporteDocenteIntegral};
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use crate::shared::access_control;

#[tauri::command]
pub async fn get_estadisticas_proyectos_x_docente(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Vec<DocenteProyectosCount>, AppError> {
    access_control::get_estadisticas_proyectos_x_docente(&state, window.label()).await
}

#[tauri::command]
pub async fn get_kpis_dashboard(
    window: Window,
    state: State<'_, AppState>,
) -> Result<KpisDashboard, AppError> {
    access_control::get_kpis_dashboard(&state, window.label()).await
}

#[tauri::command]
pub async fn get_data_exportacion_plana(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Vec<ExportData>, AppError> {
    access_control::get_data_exportacion_plana(&state, window.label()).await
}

// NEW: Improved export grouped by docente
#[tauri::command]
pub async fn get_data_exportacion_agrupada_docente(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Vec<ExportDataConProjectos>, AppError> {
    access_control::get_data_exportacion_agrupada_docente(&state, window.label()).await
}

#[tauri::command]
pub async fn write_export_file(
    window: Window,
    state: State<'_, AppState>,
    file_path: String,
    bytes: Vec<u8>,
) -> Result<(), AppError> {
    access_control::write_export_file(&state, window.label(), &file_path, bytes).await
}

// ── Reportes Integrales de Entidad ────────────────────────────────────────────

#[tauri::command]
pub async fn get_reporte_proyecto_integral(
    window: Window,
    state: State<'_, AppState>,
    id_proyecto: String,
) -> Result<ReporteProyectoIntegral, AppError> {
    access_control::get_reporte_proyecto_integral(&state, window.label(), &id_proyecto).await
}

#[tauri::command]
pub async fn get_reporte_docente_integral(
    window: Window,
    state: State<'_, AppState>,
    id_docente: String,
) -> Result<ReporteDocenteIntegral, AppError> {
    access_control::get_reporte_docente_integral(&state, window.label(), &id_docente).await
}

#[tauri::command]
pub async fn get_reportes_docentes_integral(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Vec<ReporteDocenteIntegral>, AppError> {
    access_control::get_reportes_docentes_integral(&state, window.label()).await
}

#[tauri::command]
pub async fn get_data_exportacion_grupos(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Vec<ExportDataGrupo>, AppError> {
    access_control::get_data_exportacion_grupos(&state, window.label()).await
}

#[tauri::command]
pub async fn get_data_exportacion_recursos(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Vec<ExportDataRecurso>, AppError> {
    access_control::get_data_exportacion_recursos(&state, window.label()).await
}

#[tauri::command]
pub async fn get_data_exportacion_docentes_perfil(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Vec<ExportDataDocentePerfil>, AppError> {
    access_control::get_data_exportacion_docentes_perfil(&state, window.label()).await
}

#[tauri::command]
pub async fn get_data_exportacion_proyectos_area(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Vec<ExportDataProyectoArea>, AppError> {
    access_control::get_data_exportacion_proyectos_area(&state, window.label()).await
}

#[tauri::command]
pub async fn get_proyectos_trend(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Vec<ProyectosTrendItem>, AppError> {
    access_control::get_proyectos_trend(&state, window.label()).await
}

#[tauri::command]
pub async fn get_renacyt_distribucion(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Vec<RenacytDistribucionItem>, AppError> {
    access_control::get_renacyt_distribucion(&state, window.label()).await
}
