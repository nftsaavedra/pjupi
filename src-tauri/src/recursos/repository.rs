use futures_util::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    Database,
};

use crate::recursos::models::{
    CreateEquipamientoRequest, CreateFinanciamientoRequest, CreatePatenteRequest, CreateProductoRequest,
    Equipamiento, Financiamiento, Patente, Producto,
    UpdateEquipamientoRequest, UpdateFinanciamientoRequest, UpdatePatenteRequest, UpdateProductoRequest,
};
use crate::shared::error::AppError;

// ── Patentes ──────────────────────────────────────────────────────────────────

pub async fn create_patente(db: &Database, request: CreatePatenteRequest) -> Result<Patente, AppError> {
    let patente = Patente::new(request);
    db.collection::<Patente>("patentes").insert_one(&patente).await?;
    Ok(patente)
}

pub async fn get_patentes_by_proyecto(db: &Database, proyecto_id: &str) -> Result<Vec<Patente>, AppError> {
    db.collection::<Patente>("patentes")
        .find(doc! { "proyecto_id": proyecto_id })
        .await?
        .try_collect::<Vec<_>>()
        .await
        .map_err(Into::into)
}

pub async fn get_patente_by_id(db: &Database, id_patente: &str) -> Result<Patente, AppError> {
    db.collection::<Patente>("patentes")
        .find_one(doc! { "id_patente": id_patente })
        .await?
        .ok_or_else(|| AppError::NotFound("Patente no encontrada.".to_string()))
}

pub async fn update_patente(db: &Database, id_patente: &str, request: UpdatePatenteRequest) -> Result<Patente, AppError> {
    let now = chrono::Utc::now().timestamp_millis();
    let mut update = doc! { "updated_at": now };
    if let Some(v) = request.titulo { update.insert("titulo", v); }
    if let Some(v) = request.numero_patente { update.insert("numero_patente", v); }
    if let Some(v) = request.tipo { update.insert("tipo", v); }
    if let Some(v) = request.estado { update.insert("estado", v); }
    if let Some(v) = request.fecha_solicitud { update.insert("fecha_solicitud", v); }
    if let Some(v) = request.fecha_concesion { update.insert("fecha_concesion", v); }
    if let Some(v) = request.pais { update.insert("pais", v); }
    if let Some(v) = request.entidad_concedente { update.insert("entidad_concedente", v); }
    if let Some(v) = request.descripcion { update.insert("descripcion", v); }
    db.collection::<Document>("patentes")
        .update_one(doc! { "id_patente": id_patente }, doc! { "$set": update })
        .await?;
    get_patente_by_id(db, id_patente).await
}

pub async fn delete_patente(db: &Database, id_patente: &str) -> Result<(), AppError> {
    db.collection::<Document>("patentes")
        .delete_one(doc! { "id_patente": id_patente })
        .await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn delete_patentes_by_proyecto(db: &Database, proyecto_id: &str) -> Result<(), AppError> {
    db.collection::<Document>("patentes")
        .delete_many(doc! { "proyecto_id": proyecto_id })
        .await?;
    Ok(())
}

// ── Productos ─────────────────────────────────────────────────────────────────

pub async fn create_producto(db: &Database, request: CreateProductoRequest) -> Result<Producto, AppError> {
    let producto = Producto::new(request);
    db.collection::<Producto>("productos").insert_one(&producto).await?;
    Ok(producto)
}

pub async fn get_productos_by_proyecto(db: &Database, proyecto_id: &str) -> Result<Vec<Producto>, AppError> {
    db.collection::<Producto>("productos")
        .find(doc! { "proyecto_id": proyecto_id })
        .await?
        .try_collect::<Vec<_>>()
        .await
        .map_err(Into::into)
}

pub async fn get_producto_by_id(db: &Database, id_producto: &str) -> Result<Producto, AppError> {
    db.collection::<Producto>("productos")
        .find_one(doc! { "id_producto": id_producto })
        .await?
        .ok_or_else(|| AppError::NotFound("Producto no encontrado.".to_string()))
}

pub async fn update_producto(db: &Database, id_producto: &str, request: UpdateProductoRequest) -> Result<Producto, AppError> {
    let now = chrono::Utc::now().timestamp_millis();
    let mut update = doc! { "updated_at": now };
    if let Some(v) = request.nombre { update.insert("nombre", v); }
    if let Some(v) = request.tipo { update.insert("tipo", v); }
    if let Some(v) = request.etapa { update.insert("etapa", v); }
    if let Some(v) = request.descripcion { update.insert("descripcion", v); }
    if let Some(v) = request.fecha_registro { update.insert("fecha_registro", v); }
    db.collection::<Document>("productos")
        .update_one(doc! { "id_producto": id_producto }, doc! { "$set": update })
        .await?;
    get_producto_by_id(db, id_producto).await
}

pub async fn delete_producto(db: &Database, id_producto: &str) -> Result<(), AppError> {
    db.collection::<Document>("productos")
        .delete_one(doc! { "id_producto": id_producto })
        .await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn delete_productos_by_proyecto(db: &Database, proyecto_id: &str) -> Result<(), AppError> {
    db.collection::<Document>("productos")
        .delete_many(doc! { "proyecto_id": proyecto_id })
        .await?;
    Ok(())
}

// ── Equipamientos ─────────────────────────────────────────────────────────────

pub async fn create_equipamiento(db: &Database, request: CreateEquipamientoRequest) -> Result<Equipamiento, AppError> {
    let equipamiento = Equipamiento::new(request);
    db.collection::<Equipamiento>("equipamientos").insert_one(&equipamiento).await?;
    Ok(equipamiento)
}

pub async fn get_equipamientos_by_proyecto(db: &Database, proyecto_id: &str) -> Result<Vec<Equipamiento>, AppError> {
    db.collection::<Equipamiento>("equipamientos")
        .find(doc! { "proyecto_id": proyecto_id })
        .await?
        .try_collect::<Vec<_>>()
        .await
        .map_err(Into::into)
}

pub async fn get_equipamiento_by_id(db: &Database, id_equipamiento: &str) -> Result<Equipamiento, AppError> {
    db.collection::<Equipamiento>("equipamientos")
        .find_one(doc! { "id_equipamiento": id_equipamiento })
        .await?
        .ok_or_else(|| AppError::NotFound("Equipamiento no encontrado.".to_string()))
}

pub async fn update_equipamiento(db: &Database, id_equipamiento: &str, request: UpdateEquipamientoRequest) -> Result<Equipamiento, AppError> {
    let now = chrono::Utc::now().timestamp_millis();
    let mut update = doc! { "updated_at": now };
    if let Some(v) = request.nombre { update.insert("nombre", v); }
    if let Some(v) = request.descripcion { update.insert("descripcion", v); }
    if let Some(v) = request.especificaciones { update.insert("especificaciones", v); }
    if let Some(v) = request.valor_estimado { update.insert("valor_estimado", v); }
    if let Some(v) = request.moneda { update.insert("moneda", v); }
    if let Some(v) = request.proveedor { update.insert("proveedor", v); }
    if let Some(v) = request.fecha_adquisicion { update.insert("fecha_adquisicion", v); }
    db.collection::<Document>("equipamientos")
        .update_one(doc! { "id_equipamiento": id_equipamiento }, doc! { "$set": update })
        .await?;
    get_equipamiento_by_id(db, id_equipamiento).await
}

pub async fn delete_equipamiento(db: &Database, id_equipamiento: &str) -> Result<(), AppError> {
    db.collection::<Document>("equipamientos")
        .delete_one(doc! { "id_equipamiento": id_equipamiento })
        .await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn delete_equipamientos_by_proyecto(db: &Database, proyecto_id: &str) -> Result<(), AppError> {
    db.collection::<Document>("equipamientos")
        .delete_many(doc! { "proyecto_id": proyecto_id })
        .await?;
    Ok(())
}

// ── Financiamientos ───────────────────────────────────────────────────────────

pub async fn create_financiamiento(db: &Database, request: CreateFinanciamientoRequest) -> Result<Financiamiento, AppError> {
    let financiamiento = Financiamiento::new(request);
    db.collection::<Financiamiento>("financiamientos").insert_one(&financiamiento).await?;
    Ok(financiamiento)
}

pub async fn get_financiamientos_by_proyecto(db: &Database, proyecto_id: &str) -> Result<Vec<Financiamiento>, AppError> {
    db.collection::<Financiamiento>("financiamientos")
        .find(doc! { "proyecto_id": proyecto_id })
        .await?
        .try_collect::<Vec<_>>()
        .await
        .map_err(Into::into)
}

pub async fn get_financiamiento_by_id(db: &Database, id_financiamiento: &str) -> Result<Financiamiento, AppError> {
    db.collection::<Financiamiento>("financiamientos")
        .find_one(doc! { "id_financiamiento": id_financiamiento })
        .await?
        .ok_or_else(|| AppError::NotFound("Financiamiento no encontrado.".to_string()))
}

pub async fn update_financiamiento(db: &Database, id_financiamiento: &str, request: UpdateFinanciamientoRequest) -> Result<Financiamiento, AppError> {
    let now = chrono::Utc::now().timestamp_millis();
    let mut update = doc! { "updated_at": now };
    if let Some(v) = request.entidad_financiadora { update.insert("entidad_financiadora", v); }
    if let Some(v) = request.tipo { update.insert("tipo", v); }
    if let Some(v) = request.monto { update.insert("monto", v); }
    if let Some(v) = request.moneda { update.insert("moneda", v); }
    if let Some(v) = request.fecha_inicio { update.insert("fecha_inicio", v); }
    if let Some(v) = request.fecha_fin { update.insert("fecha_fin", v); }
    if let Some(v) = request.descripcion { update.insert("descripcion", v); }
    if let Some(v) = request.estado_financiero { update.insert("estado_financiero", v); }
    db.collection::<Document>("financiamientos")
        .update_one(doc! { "id_financiamiento": id_financiamiento }, doc! { "$set": update })
        .await?;
    get_financiamiento_by_id(db, id_financiamiento).await
}

pub async fn delete_financiamiento(db: &Database, id_financiamiento: &str) -> Result<(), AppError> {
    db.collection::<Document>("financiamientos")
        .delete_one(doc! { "id_financiamiento": id_financiamiento })
        .await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn delete_financiamientos_by_proyecto(db: &Database, proyecto_id: &str) -> Result<(), AppError> {
    db.collection::<Document>("financiamientos")
        .delete_many(doc! { "proyecto_id": proyecto_id })
        .await?;
    Ok(())
}
