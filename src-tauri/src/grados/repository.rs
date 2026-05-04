use futures_util::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::Database;

use crate::grados::models::{CreateGradoRequest, EliminarGradoResultado, GradoAcademico};
use crate::shared::error::AppError;

pub async fn create_grado(db: &Database, request: CreateGradoRequest) -> Result<GradoAcademico, AppError> {
    let grado = GradoAcademico::new(request);
    db.collection::<GradoAcademico>("grados").insert_one(&grado).await?;
    Ok(grado)
}

pub async fn get_all_grados(db: &Database) -> Result<Vec<GradoAcademico>, AppError> {
    let mut grados = db.collection::<GradoAcademico>("grados")
        .find(doc! {})
        .await?
        .try_collect::<Vec<_>>()
        .await?;
    grados.sort_by(|a, b| a.nombre.to_lowercase().cmp(&b.nombre.to_lowercase()));
    Ok(grados)
}

pub async fn get_grado_by_id(db: &Database, id_grado: &str) -> Result<GradoAcademico, AppError> {
    db.collection::<GradoAcademico>("grados")
        .find_one(doc! { "id_grado": id_grado })
        .await?
        .ok_or_else(|| AppError::NotFound("Grado no encontrado.".to_string()))
}

pub async fn update_grado(db: &Database, id_grado: &str, request: CreateGradoRequest) -> Result<GradoAcademico, AppError> {
    db.collection::<GradoAcademico>("grados")
        .update_one(
            doc! { "id_grado": id_grado },
            doc! { "$set": { "nombre": request.nombre, "descripcion": request.descripcion } },
        )
        .await?;
    get_grado_by_id(db, id_grado).await
}

pub async fn delete_grado(db: &Database, id_grado: &str) -> Result<EliminarGradoResultado, AppError> {
    let docentes_relacionados = db.collection::<Document>("docentes")
        .count_documents(doc! { "id_grado": id_grado })
        .await?;

    if docentes_relacionados > 0 {
        db.collection::<Document>("grados")
            .update_one(doc! { "id_grado": id_grado }, doc! { "$set": { "activo": 0i64 } })
            .await?;
        return Ok(EliminarGradoResultado {
            accion: "desactivado".to_string(),
            mensaje: "El grado está relacionado con docentes. Se desactivó en lugar de eliminarse.".to_string(),
        });
    }

    db.collection::<Document>("grados")
        .delete_one(doc! { "id_grado": id_grado })
        .await?;

    Ok(EliminarGradoResultado {
        accion: "eliminado".to_string(),
        mensaje: "Grado eliminado correctamente.".to_string(),
    })
}

pub async fn reactivar_grado(db: &Database, id_grado: &str) -> Result<GradoAcademico, AppError> {
    db.collection::<Document>("grados")
        .update_one(doc! { "id_grado": id_grado }, doc! { "$set": { "activo": 1i64 } })
        .await?;
    get_grado_by_id(db, id_grado).await
}
