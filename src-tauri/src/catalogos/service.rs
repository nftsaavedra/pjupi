use crate::catalogos::models::{CatalogoItem, CreateCatalogoRequest, EliminarCatalogoResultado};
use crate::catalogos::repository;
use crate::shared::error::AppError;
use crate::shared::state::AppState;

pub async fn create(state: &AppState, request: CreateCatalogoRequest) -> Result<CatalogoItem, AppError> {
    repository::create_catalogo(state.mongo_db()?, request).await
}

pub async fn get_by_tipo(state: &AppState, tipo: &str) -> Result<Vec<CatalogoItem>, AppError> {
    repository::get_catalogos_by_tipo(state.mongo_db()?, tipo).await
}

pub async fn get_all_by_tipo(state: &AppState, tipo: &str) -> Result<Vec<CatalogoItem>, AppError> {
    repository::get_all_catalogos(state.mongo_db()?, tipo).await
}

pub async fn update(state: &AppState, id: &str, request: CreateCatalogoRequest) -> Result<CatalogoItem, AppError> {
    repository::update_catalogo(state.mongo_db()?, id, request).await
}

pub async fn delete(state: &AppState, id: &str) -> Result<EliminarCatalogoResultado, AppError> {
    repository::delete_catalogo(state.mongo_db()?, id).await
}

pub async fn reactivate(state: &AppState, id: &str) -> Result<CatalogoItem, AppError> {
    repository::reactivar_catalogo(state.mongo_db()?, id).await
}
