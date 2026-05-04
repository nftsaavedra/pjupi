use crate::proyectos::repository;
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use crate::reportes::models::{DocenteProyectosCount, ExportData, ExportDataConProjectos, KpisDashboard};

#[allow(dead_code)]
pub async fn get_estadisticas_x_docente(state: &AppState) -> Result<Vec<DocenteProyectosCount>, AppError> {
    let db = state.mongo_db()?;
    repository::get_estadisticas_proyectos_x_docente(db).await
}

#[allow(dead_code)]
pub async fn get_kpis(state: &AppState) -> Result<KpisDashboard, AppError> {
    let db = state.mongo_db()?;
    repository::get_kpis_dashboard(db).await
}

#[allow(dead_code)]
pub async fn get_exportacion_plana(state: &AppState) -> Result<Vec<ExportData>, AppError> {
    let db = state.mongo_db()?;
    repository::get_data_exportacion_plana(db).await
}

#[allow(dead_code)]
pub async fn get_exportacion_agrupada(state: &AppState) -> Result<Vec<ExportDataConProjectos>, AppError> {
    let db = state.mongo_db()?;
    repository::get_data_exportacion_agrupada_docente(db).await
}
