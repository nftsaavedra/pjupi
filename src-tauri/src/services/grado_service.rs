use crate::domain::grado::{CreateGradoRequest, EliminarGradoResultado, GradoAcademico};
use crate::error::AppError;
use crate::infrastructure::mongo_repo;
use crate::state::AppState;

pub async fn get_all(state: &AppState) -> Result<Vec<GradoAcademico>, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_all_grados(db).await
}

pub async fn create(state: &AppState, request: CreateGradoRequest) -> Result<GradoAcademico, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::create_grado(db, request).await
}

pub async fn update(state: &AppState, id_grado: &str, request: CreateGradoRequest) -> Result<GradoAcademico, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::update_grado(db, id_grado, request).await
}

pub async fn delete(state: &AppState, id_grado: &str) -> Result<EliminarGradoResultado, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::delete_grado(db, id_grado).await
}

pub async fn reactivate(state: &AppState, id_grado: &str) -> Result<GradoAcademico, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::reactivar_grado(db, id_grado).await
}
