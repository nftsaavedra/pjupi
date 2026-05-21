use futures_util::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Database;

use crate::catalogos::models::{CatalogoItem, CreateCatalogoRequest, EliminarCatalogoResultado};
use crate::shared::error::AppError;

pub async fn create_catalogo(db: &Database, request: CreateCatalogoRequest) -> Result<CatalogoItem, AppError> {
    let item = CatalogoItem::new(request);
    db.collection::<CatalogoItem>("catalogos").insert_one(&item).await?;
    Ok(item)
}

pub async fn get_catalogos_by_tipo(db: &Database, tipo: &str) -> Result<Vec<CatalogoItem>, AppError> {
    let mut items = db.collection::<CatalogoItem>("catalogos")
        .find(doc! { "tipo": tipo, "activo": 1i64 })
        .await?
        .try_collect::<Vec<_>>()
        .await?;
    items.sort_by(|a, b|
        a.orden.unwrap_or(999).cmp(&b.orden.unwrap_or(999))
            .then_with(|| a.nombre.to_lowercase().cmp(&b.nombre.to_lowercase()))
    );
    Ok(items)
}

pub async fn get_all_catalogos(db: &Database, tipo: &str) -> Result<Vec<CatalogoItem>, AppError> {
    let mut items = db.collection::<CatalogoItem>("catalogos")
        .find(doc! { "tipo": tipo })
        .await?
        .try_collect::<Vec<_>>()
        .await?;
    items.sort_by(|a, b|
        a.nombre.to_lowercase().cmp(&b.nombre.to_lowercase())
    );
    Ok(items)
}

pub async fn get_catalogo_by_id(db: &Database, id: &str) -> Result<CatalogoItem, AppError> {
    db.collection::<CatalogoItem>("catalogos")
        .find_one(doc! { "id_catalogo": id })
        .await?
        .ok_or_else(|| AppError::NotFound("Catálogo no encontrado.".to_string()))
}

pub async fn update_catalogo(db: &Database, id: &str, request: CreateCatalogoRequest) -> Result<CatalogoItem, AppError> {
    let now = crate::shared::time::now_ms();
    db.collection::<CatalogoItem>("catalogos")
        .update_one(
            doc! { "id_catalogo": id },
            doc! { "$set": {
                "codigo": request.codigo,
                "nombre": request.nombre,
                "descripcion": request.descripcion,
                "orden": request.orden,
                "updated_at": now,
            }},
        )
        .await?;
    get_catalogo_by_id(db, id).await
}

pub async fn delete_catalogo(db: &Database, id: &str) -> Result<EliminarCatalogoResultado, AppError> {
    db.collection::<CatalogoItem>("catalogos")
        .update_one(doc! { "id_catalogo": id }, doc! { "$set": { "activo": 0i64 } })
        .await?;
    Ok(EliminarCatalogoResultado {
        accion: "desactivado".to_string(),
        mensaje: "Catálogo desactivado correctamente.".to_string(),
    })
}

pub async fn reactivar_catalogo(db: &Database, id: &str) -> Result<CatalogoItem, AppError> {
    db.collection::<CatalogoItem>("catalogos")
        .update_one(doc! { "id_catalogo": id }, doc! { "$set": { "activo": 1i64 } })
        .await?;
    get_catalogo_by_id(db, id).await
}

pub async fn seed_catalogos(db: &Database) -> Result<(), AppError> {
    let count = db.collection::<CatalogoItem>("catalogos").count_documents(doc! {}).await?;
    if count > 0 { return Ok(()); }

    let seed = vec![
        ("tipo_patente", "invencion", "Invención", 1),
        ("tipo_patente", "modelo_utilidad", "Modelo de Utilidad", 2),
        ("tipo_patente", "diseno_industrial", "Diseño Industrial", 3),
        ("estado_patente", "solicitada", "Solicitada", 1),
        ("estado_patente", "en_examen", "En Examen", 2),
        ("estado_patente", "concedida", "Concedida", 3),
        ("estado_patente", "rechazada", "Rechazada", 4),
        ("tipo_producto", "software", "Software", 1),
        ("tipo_producto", "prototipo", "Prototipo", 2),
        ("tipo_producto", "metodologia", "Metodología", 3),
        ("tipo_producto", "norma", "Norma Técnica", 4),
        ("tipo_producto", "base_datos", "Base de Datos", 5),
        ("etapa_producto", "conceptualizacion", "Conceptualización", 1),
        ("etapa_producto", "prototipo", "Prototipo", 2),
        ("etapa_producto", "validacion", "Validación", 3),
        ("etapa_producto", "produccion", "Producción", 4),
        ("etapa_producto", "comercializacion", "Comercialización", 5),
        ("tipo_financiamiento", "nacional", "Nacional", 1),
        ("tipo_financiamiento", "internacional", "Internacional", 2),
        ("tipo_financiamiento", "propio", "Propio / Autofinanciado", 3),
        ("tipo_financiamiento", "concursable", "Fondos Concursables", 4),
        ("estado_financiero", "aprobado", "Aprobado", 1),
        ("estado_financiero", "desembolsado", "Desembolsado", 2),
        ("estado_financiero", "en_proceso", "En Proceso", 3),
        ("estado_financiero", "finalizado", "Finalizado", 4),
        ("estado_financiero", "cancelado", "Cancelado", 5),
        ("moneda", "PEN", "Sol Peruano (PEN)", 1),
        ("moneda", "USD", "Dólar (USD)", 2),
        ("moneda", "EUR", "Euro (EUR)", 3),
    ];

    for (tipo, codigo, nombre, orden) in seed {
        create_catalogo(db, CreateCatalogoRequest {
            tipo: tipo.to_string(),
            codigo: codigo.to_string(),
            nombre: nombre.to_string(),
            descripcion: None,
            orden: Some(orden),
        }).await?;
    }
    Ok(())
}
