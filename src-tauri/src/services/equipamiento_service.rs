use crate::domain::equipamiento::{CreateEquipamientoRequest, Equipamiento, UpdateEquipamientoRequest};
use crate::error::AppError;
use crate::infrastructure::mongo_repo;
use crate::state::AppState;

pub async fn create(state: &AppState, request: CreateEquipamientoRequest) -> Result<Equipamiento, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::create_equipamiento(db, request).await
}

pub async fn get_by_proyecto(state: &AppState, proyecto_id: &str) -> Result<Vec<Equipamiento>, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_equipamientos_by_proyecto(db, proyecto_id).await
}

#[allow(dead_code)]
pub async fn get_by_id(state: &AppState, id_equipamiento: &str) -> Result<Equipamiento, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_equipamiento_by_id(db, id_equipamiento).await
}

pub async fn update(state: &AppState, id_equipamiento: &str, request: UpdateEquipamientoRequest) -> Result<Equipamiento, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::update_equipamiento(db, id_equipamiento, request).await
}

pub async fn delete(state: &AppState, id_equipamiento: &str) -> Result<(), AppError> {
    let db = state.mongo_db()?;
    mongo_repo::delete_equipamiento(db, id_equipamiento).await
}
