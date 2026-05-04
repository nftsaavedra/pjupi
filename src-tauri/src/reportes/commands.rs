use std::path::Path;

use tauri::{State, Window};
use crate::reportes::models::{DocenteProyectosCount, ExportData, ExportDataConProjectos, KpisDashboard};
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
    file_path: String,
    bytes: Vec<u8>,
) -> Result<(), AppError> {
    let trimmed_path = file_path.trim();
    if trimmed_path.is_empty() {
        return Err(AppError::ConfigurationError(
            "La ruta de exportacion es invalida.".to_string(),
        ));
    }

    if let Some(parent) = Path::new(trimmed_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|error| {
                AppError::InternalError(format!(
                    "No se pudo preparar la carpeta de exportacion: {error}"
                ))
            })?;
        }
    }

    std::fs::write(trimmed_path, bytes).map_err(|error| {
        AppError::InternalError(format!("No se pudo guardar el archivo exportado: {error}"))
    })
}
