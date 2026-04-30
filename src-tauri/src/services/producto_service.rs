use crate::domain::producto::{CreateProductoRequest, Producto, UpdateProductoRequest};
use crate::error::AppError;
use crate::infrastructure::mongo_repo;
use crate::state::AppState;

pub async fn create(state: &AppState, request: CreateProductoRequest) -> Result<Producto, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::create_producto(db, request).await
}

pub async fn get_by_proyecto(state: &AppState, proyecto_id: &str) -> Result<Vec<Producto>, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_productos_by_proyecto(db, proyecto_id).await
}

#[allow(dead_code)]
pub async fn get_by_id(state: &AppState, id_producto: &str) -> Result<Producto, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_producto_by_id(db, id_producto).await
}

pub async fn update(state: &AppState, id_producto: &str, request: UpdateProductoRequest) -> Result<Producto, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::update_producto(db, id_producto, request).await
}

pub async fn delete(state: &AppState, id_producto: &str) -> Result<(), AppError> {
    let db = state.mongo_db()?;
    mongo_repo::delete_producto(db, id_producto).await
}
