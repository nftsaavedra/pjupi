use crate::domain::financiamiento::{CreateFinanciamientoRequest, Financiamiento, UpdateFinanciamientoRequest};
use crate::error::AppError;
use crate::infrastructure::mongo_repo;
use crate::state::AppState;

pub async fn create(state: &AppState, request: CreateFinanciamientoRequest) -> Result<Financiamiento, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::create_financiamiento(db, request).await
}

pub async fn get_by_proyecto(state: &AppState, proyecto_id: &str) -> Result<Vec<Financiamiento>, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_financiamientos_by_proyecto(db, proyecto_id).await
}

#[allow(dead_code)]
pub async fn get_by_id(state: &AppState, id_financiamiento: &str) -> Result<Financiamiento, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_financiamiento_by_id(db, id_financiamiento).await
}

pub async fn update(state: &AppState, id_financiamiento: &str, request: UpdateFinanciamientoRequest) -> Result<Financiamiento, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::update_financiamiento(db, id_financiamiento, request).await
}

pub async fn delete(state: &AppState, id_financiamiento: &str) -> Result<(), AppError> {
    let db = state.mongo_db()?;
    mongo_repo::delete_financiamiento(db, id_financiamiento).await
}
