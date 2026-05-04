use mongodb::{
    bson::{doc, Document},
    options::IndexOptions,
    Client, Database, IndexModel,
};

use crate::shared::config::DatabaseConfig;
use crate::shared::error::AppError;

pub async fn init_mongo(config: &DatabaseConfig) -> Result<Database, AppError> {
    let uri = config.mongodb_uri.as_deref().ok_or_else(|| {
        AppError::ConfigurationError("Falta configurar PJUPI_MONGODB_URI para usar MongoDB.".to_string())
    })?;

    let client = Client::with_uri_str(uri).await?;
    let database = client.database(&config.mongodb_db_name);
    ensure_indexes(&database).await?;
    Ok(database)
}

pub async fn ensure_indexes(db: &Database) -> Result<(), AppError> {
    db.collection::<Document>("grados")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "id_grado": 1 })
                .options(Some(IndexOptions::builder().unique(true).build()))
                .build(),
        )
        .await?;
    db.collection::<Document>("grados")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "nombre": 1 })
                .options(Some(IndexOptions::builder().unique(true).build()))
                .build(),
        )
        .await?;

    db.collection::<Document>("docentes")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "id_docente": 1 })
                .options(Some(IndexOptions::builder().unique(true).build()))
                .build(),
        )
        .await?;
    db.collection::<Document>("docentes")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "dni": 1 })
                .options(Some(IndexOptions::builder().unique(true).build()))
                .build(),
        )
        .await?;
    db.collection::<Document>("docentes")
        .create_index(IndexModel::builder().keys(doc! { "renacyt_id_investigador": 1 }).build())
        .await?;
    db.collection::<Document>("docentes")
        .create_index(IndexModel::builder().keys(doc! { "renacyt_codigo_registro": 1 }).build())
        .await?;

    db.collection::<Document>("proyectos")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "id_proyecto": 1 })
                .options(Some(IndexOptions::builder().unique(true).build()))
                .build(),
        )
        .await?;

    db.collection::<Document>("participaciones")
        .create_index(IndexModel::builder().keys(doc! { "id_proyecto": 1 }).build())
        .await?;
    db.collection::<Document>("participaciones")
        .create_index(IndexModel::builder().keys(doc! { "id_docente": 1 }).build())
        .await?;

    db.collection::<Document>("usuarios")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "id_usuario": 1 })
                .options(Some(IndexOptions::builder().unique(true).build()))
                .build(),
        )
        .await?;
    db.collection::<Document>("usuarios")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "username": 1 })
                .options(Some(IndexOptions::builder().unique(true).build()))
                .build(),
        )
        .await?;

    // --- Publicaciones (Pure sync) ---
    db.collection::<Document>("publicaciones")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "pure_uuid": 1 })
                .options(Some(IndexOptions::builder().unique(true).build()))
                .build(),
        )
        .await?;
    db.collection::<Document>("publicaciones")
        .create_index(IndexModel::builder().keys(doc! { "docente_id": 1 }).build())
        .await?;
    db.collection::<Document>("publicaciones")
        .create_index(IndexModel::builder().keys(doc! { "proyecto_id": 1 }).build())
        .await?;

    // --- Patentes ---
    db.collection::<Document>("patentes")
        .create_index(IndexModel::builder().keys(doc! { "proyecto_id": 1 }).build())
        .await?;
    db.collection::<Document>("patentes")
        .create_index(IndexModel::builder().keys(doc! { "docente_id": 1 }).build())
        .await?;
    db.collection::<Document>("patentes")
        .create_index(IndexModel::builder().keys(doc! { "numero_patente": 1 }).build())
        .await?;

    // --- Productos ---
    db.collection::<Document>("productos")
        .create_index(IndexModel::builder().keys(doc! { "proyecto_id": 1 }).build())
        .await?;
    db.collection::<Document>("productos")
        .create_index(IndexModel::builder().keys(doc! { "docente_id": 1 }).build())
        .await?;

    // --- Equipamientos ---
    db.collection::<Document>("equipamientos")
        .create_index(IndexModel::builder().keys(doc! { "proyecto_id": 1 }).build())
        .await?;

    // --- Financiamientos ---
    db.collection::<Document>("financiamientos")
        .create_index(IndexModel::builder().keys(doc! { "proyecto_id": 1 }).build())
        .await?;

    // --- Grupos de investigación ---
    db.collection::<Document>("grupos_investigacion")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "id_grupo": 1 })
                .options(Some(IndexOptions::builder().unique(true).build()))
                .build(),
        )
        .await?;
    db.collection::<Document>("grupos_investigacion")
        .create_index(IndexModel::builder().keys(doc! { "coordinador_id": 1 }).build())
        .await?;

    Ok(())
}
