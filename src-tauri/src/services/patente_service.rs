use crate::domain::patente::{CreatePatenteRequest, Patente, UpdatePatenteRequest};
use crate::error::AppError;
use crate::infrastructure::mongo_repo;
use crate::state::AppState;

pub async fn create(state: &AppState, request: CreatePatenteRequest) -> Result<Patente, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::create_patente(db, request).await
}

pub async fn get_by_proyecto(state: &AppState, proyecto_id: &str) -> Result<Vec<Patente>, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_patentes_by_proyecto(db, proyecto_id).await
}

#[allow(dead_code)]
pub async fn get_by_id(state: &AppState, id_patente: &str) -> Result<Patente, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_patente_by_id(db, id_patente).await
}

pub async fn update(state: &AppState, id_patente: &str, request: UpdatePatenteRequest) -> Result<Patente, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::update_patente(db, id_patente, request).await
}

pub async fn delete(state: &AppState, id_patente: &str) -> Result<(), AppError> {
    let db = state.mongo_db()?;
    mongo_repo::delete_patente(db, id_patente).await
}
