use crate::recursos::models::{
    CreateEquipamientoRequest, CreateFinanciamientoRequest, CreatePatenteRequest, CreateProductoRequest,
    Equipamiento, Financiamiento, Patente, Producto,
    UpdateEquipamientoRequest, UpdateFinanciamientoRequest, UpdatePatenteRequest, UpdateProductoRequest,
};
use crate::recursos::repository;
use crate::shared::error::AppError;
use crate::shared::state::AppState;

pub async fn create_patente(state: &AppState, request: CreatePatenteRequest) -> Result<Patente, AppError> {
    let db = state.mongo_db()?;
    repository::create_patente(db, request).await
}
pub async fn get_patentes_by_proyecto(state: &AppState, proyecto_id: &str) -> Result<Vec<Patente>, AppError> {
    let db = state.mongo_db()?;
    repository::get_patentes_by_proyecto(db, proyecto_id).await
}
pub async fn update_patente(state: &AppState, id_patente: &str, request: UpdatePatenteRequest) -> Result<Patente, AppError> {
    let db = state.mongo_db()?;
    repository::update_patente(db, id_patente, request).await
}
pub async fn delete_patente(state: &AppState, id_patente: &str) -> Result<(), AppError> {
    let db = state.mongo_db()?;
    repository::delete_patente(db, id_patente).await
}

pub async fn create_producto(state: &AppState, request: CreateProductoRequest) -> Result<Producto, AppError> {
    let db = state.mongo_db()?;
    repository::create_producto(db, request).await
}
pub async fn get_productos_by_proyecto(state: &AppState, proyecto_id: &str) -> Result<Vec<Producto>, AppError> {
    let db = state.mongo_db()?;
    repository::get_productos_by_proyecto(db, proyecto_id).await
}
pub async fn update_producto(state: &AppState, id_producto: &str, request: UpdateProductoRequest) -> Result<Producto, AppError> {
    let db = state.mongo_db()?;
    repository::update_producto(db, id_producto, request).await
}
pub async fn delete_producto(state: &AppState, id_producto: &str) -> Result<(), AppError> {
    let db = state.mongo_db()?;
    repository::delete_producto(db, id_producto).await
}

pub async fn create_equipamiento(state: &AppState, request: CreateEquipamientoRequest) -> Result<Equipamiento, AppError> {
    let db = state.mongo_db()?;
    repository::create_equipamiento(db, request).await
}
pub async fn get_equipamientos_by_proyecto(state: &AppState, proyecto_id: &str) -> Result<Vec<Equipamiento>, AppError> {
    let db = state.mongo_db()?;
    repository::get_equipamientos_by_proyecto(db, proyecto_id).await
}
pub async fn update_equipamiento(state: &AppState, id_equipamiento: &str, request: UpdateEquipamientoRequest) -> Result<Equipamiento, AppError> {
    let db = state.mongo_db()?;
    repository::update_equipamiento(db, id_equipamiento, request).await
}
pub async fn delete_equipamiento(state: &AppState, id_equipamiento: &str) -> Result<(), AppError> {
    let db = state.mongo_db()?;
    repository::delete_equipamiento(db, id_equipamiento).await
}

pub async fn create_financiamiento(state: &AppState, request: CreateFinanciamientoRequest) -> Result<Financiamiento, AppError> {
    let db = state.mongo_db()?;
    repository::create_financiamiento(db, request).await
}
pub async fn get_financiamientos_by_proyecto(state: &AppState, proyecto_id: &str) -> Result<Vec<Financiamiento>, AppError> {
    let db = state.mongo_db()?;
    repository::get_financiamientos_by_proyecto(db, proyecto_id).await
}
pub async fn update_financiamiento(state: &AppState, id_financiamiento: &str, request: UpdateFinanciamientoRequest) -> Result<Financiamiento, AppError> {
    let db = state.mongo_db()?;
    repository::update_financiamiento(db, id_financiamiento, request).await
}
pub async fn delete_financiamiento(state: &AppState, id_financiamiento: &str) -> Result<(), AppError> {
    let db = state.mongo_db()?;
    repository::delete_financiamiento(db, id_financiamiento).await
}
