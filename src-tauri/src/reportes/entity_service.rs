use crate::reportes::entity_reports::{ReporteProyectoIntegral, ReporteDocenteIntegral};
use crate::reportes::entity_repository;
use crate::shared::error::AppError;
use crate::shared::state::AppState;

pub async fn get_reporte_proyecto(state: &AppState, id_proyecto: &str) -> Result<ReporteProyectoIntegral, AppError> {
    let db = state.mongo_db()?;
    entity_repository::build_reporte_proyecto_integral(db, id_proyecto).await
}

pub async fn get_reporte_docente(state: &AppState, id_docente: &str) -> Result<ReporteDocenteIntegral, AppError> {
    let db = state.mongo_db()?;
    entity_repository::build_reporte_docente_integral(db, id_docente).await
}

pub async fn get_reportes_docentes(state: &AppState) -> Result<Vec<ReporteDocenteIntegral>, AppError> {
    let db = state.mongo_db()?;
    entity_repository::build_reportes_docentes_integral(db).await
}
