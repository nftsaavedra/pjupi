use mongodb::{
    bson::{doc, Document},
    Database,
};
use sqlx::{FromRow, SqlitePool};

use crate::domain::docente::Docente;
use crate::domain::grado::GradoAcademico;
use crate::domain::proyecto::Proyecto;
use crate::domain::usuario::UsuarioConPassword;
use crate::error::AppError;

#[derive(Debug, FromRow, serde::Serialize, serde::Deserialize)]
struct ParticipacionSqlite {
    id_proyecto: String,
    id_docente: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ParticipacionMongo {
    #[serde(rename = "_id")]
    id: String,
    id_proyecto: String,
    id_docente: String,
}

pub async fn migrate_sqlite_to_mongodb(pool: &SqlitePool, db: &Database) -> Result<(), AppError> {
    let meta = db.collection::<Document>("system_meta");
    let existing_meta = meta.find_one(doc! { "_id": "sqlite_migration_v1" }).await?;
    if existing_meta.is_some() {
        return Ok(());
    }

    let grado_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM grado_academico")
        .fetch_one(pool)
        .await?;
    let docente_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM docente")
        .fetch_one(pool)
        .await?;
    let proyecto_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM proyecto")
        .fetch_one(pool)
        .await?;
    let participacion_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM participacion")
        .fetch_one(pool)
        .await?;
    let usuario_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM usuario")
        .fetch_one(pool)
        .await?;

    let target_has_data = db.collection::<Document>("grados").count_documents(doc! {}).await? > 0
        || db.collection::<Document>("docentes").count_documents(doc! {}).await? > 0
        || db.collection::<Document>("proyectos").count_documents(doc! {}).await? > 0
        || db.collection::<Document>("participaciones").count_documents(doc! {}).await? > 0
        || db.collection::<Document>("usuarios").count_documents(doc! {}).await? > 0;

    if target_has_data {
        meta.insert_one(doc! {
            "_id": "sqlite_migration_v1",
            "status": "skipped_existing_target_data",
        }).await?;
        return Ok(());
    }

    if grado_count.0 == 0 && docente_count.0 == 0 && proyecto_count.0 == 0 && participacion_count.0 == 0 && usuario_count.0 == 0 {
        meta.insert_one(doc! {
            "_id": "sqlite_migration_v1",
            "status": "skipped_empty_source",
        }).await?;
        return Ok(());
    }

    let grados: Vec<GradoAcademico> = sqlx::query_as(
        "SELECT id_grado, nombre, descripcion, activo FROM grado_academico"
    )
    .fetch_all(pool)
    .await?;

    let docentes: Vec<Docente> = sqlx::query_as(
        "SELECT id_docente, dni, id_grado, nombres_apellidos, nombres, apellido_paterno, apellido_materno, activo FROM docente"
    )
    .fetch_all(pool)
    .await?;

    let proyectos: Vec<Proyecto> = sqlx::query_as(
        "SELECT id_proyecto, titulo_proyecto, activo FROM proyecto"
    )
    .fetch_all(pool)
    .await?;

    let participaciones: Vec<ParticipacionSqlite> = sqlx::query_as(
        "SELECT id_proyecto, id_docente FROM participacion"
    )
    .fetch_all(pool)
    .await?;

    let usuarios: Vec<UsuarioConPassword> = sqlx::query_as(
        "SELECT id_usuario, username, nombre_completo, rol, password_hash, activo FROM usuario"
    )
    .fetch_all(pool)
    .await?;

    let grados_collection = db.collection::<GradoAcademico>("grados");
    for grado in grados {
        grados_collection
            .replace_one(doc! { "id_grado": &grado.id_grado }, grado)
            .upsert(true)
            .await?;
    }

    let docentes_collection = db.collection::<Docente>("docentes");
    for docente in docentes {
        docentes_collection
            .replace_one(doc! { "id_docente": &docente.id_docente }, docente)
            .upsert(true)
            .await?;
    }

    let proyectos_collection = db.collection::<Proyecto>("proyectos");
    for proyecto in proyectos {
        proyectos_collection
            .replace_one(doc! { "id_proyecto": &proyecto.id_proyecto }, proyecto)
            .upsert(true)
            .await?;
    }

    let participaciones_collection = db.collection::<ParticipacionMongo>("participaciones");
    for participacion in participaciones {
        let record = ParticipacionMongo {
            id: format!("{}:{}", participacion.id_proyecto, participacion.id_docente),
            id_proyecto: participacion.id_proyecto,
            id_docente: participacion.id_docente,
        };

        participaciones_collection
            .replace_one(doc! { "_id": &record.id }, record)
            .upsert(true)
            .await?;
    }

    let usuarios_collection = db.collection::<UsuarioConPassword>("usuarios");
    for usuario in usuarios {
        usuarios_collection
            .replace_one(doc! { "id_usuario": &usuario.id_usuario }, usuario)
            .upsert(true)
            .await?;
    }

    meta.insert_one(doc! {
        "_id": "sqlite_migration_v1",
        "status": "completed",
        "grados": grado_count.0,
        "docentes": docente_count.0,
        "proyectos": proyecto_count.0,
        "participaciones": participacion_count.0,
        "usuarios": usuario_count.0,
    }).await?;

    Ok(())
}