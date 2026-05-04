use futures_util::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Database;

use crate::grupos::models::{GrupoInvestigacion, UpdateGrupoInvestigacionRequest};
use crate::shared::error::AppError;

pub async fn get_all_grupos(db: &Database) -> Result<Vec<GrupoInvestigacion>, AppError> {
    let grupos = db.collection::<GrupoInvestigacion>("grupos_investigacion")
        .find(doc! {})
        .await?
        .try_collect::<Vec<_>>()
        .await?;
    Ok(grupos)
}

pub async fn create_grupo(db: &Database, grupo: GrupoInvestigacion) -> Result<GrupoInvestigacion, AppError> {
    let _ = db.collection::<GrupoInvestigacion>("grupos_investigacion")
        .insert_one(&grupo)
        .await?;
    Ok(grupo)
}

pub async fn get_grupo_by_id(db: &Database, id_grupo: &str) -> Result<GrupoInvestigacion, AppError> {
    let grupo = db.collection::<GrupoInvestigacion>("grupos_investigacion")
        .find_one(doc! { "id_grupo": id_grupo })
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Grupo con ID {} no encontrado", id_grupo)))?;
    Ok(grupo)
}

pub async fn update_grupo(db: &Database, id_grupo: &str, request: UpdateGrupoInvestigacionRequest) -> Result<GrupoInvestigacion, AppError> {
    db.collection::<GrupoInvestigacion>("grupos_investigacion")
        .update_one(
            doc! { "id_grupo": id_grupo },
            doc! {
                "$set": {
                    "nombre": request.nombre,
                    "descripcion": request.descripcion,
                    "coordinador_id": request.coordinador_id,
                    "lineas_investigacion": request.lineas_investigacion,
                    "updated_at": chrono::Utc::now().timestamp_millis(),
                }
            },
        )
        .await?;

    get_grupo_by_id(db, id_grupo).await
}

pub async fn delete_grupo(db: &Database, id_grupo: &str) -> Result<(), AppError> {
    db.collection::<GrupoInvestigacion>("grupos_investigacion")
        .delete_one(doc! { "id_grupo": id_grupo })
        .await?;
    Ok(())
}
