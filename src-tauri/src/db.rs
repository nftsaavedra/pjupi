use mongodb::{Client, Database};
use sqlx::{query, SqlitePool};

use crate::config::DatabaseConfig;
use crate::error::AppError;
use crate::infrastructure::mongo_repo;

pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    query(r#"
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS grado_academico (
            id_grado TEXT PRIMARY KEY,
            nombre VARCHAR(50) UNIQUE NOT NULL,
            descripcion TEXT,
            activo INTEGER NOT NULL DEFAULT 1
        );

        CREATE TABLE IF NOT EXISTS docente (
            id_docente TEXT PRIMARY KEY,
            dni VARCHAR(8) UNIQUE NOT NULL,
            id_grado TEXT NOT NULL,
            nombres_apellidos VARCHAR(150) NOT NULL,
            nombres VARCHAR(100),
            apellido_paterno VARCHAR(80),
            apellido_materno VARCHAR(80),
            activo INTEGER NOT NULL DEFAULT 1,
            renacyt_codigo_registro VARCHAR(20),
            renacyt_id_investigador VARCHAR(20),
            renacyt_nivel VARCHAR(20),
            renacyt_grupo VARCHAR(20),
            renacyt_condicion VARCHAR(80),
            renacyt_fecha_informe_calificacion INTEGER,
            renacyt_fecha_registro INTEGER,
            renacyt_fecha_ultima_revision INTEGER,
            renacyt_orcid VARCHAR(40),
            renacyt_scopus_author_id VARCHAR(40),
            renacyt_fecha_ultima_sincronizacion INTEGER,
            renacyt_ficha_url TEXT,
            renacyt_formaciones_academicas_json TEXT,
            FOREIGN KEY (id_grado) REFERENCES grado_academico (id_grado)
        );

        CREATE TABLE IF NOT EXISTS proyecto (
            id_proyecto TEXT PRIMARY KEY,
            titulo_proyecto TEXT NOT NULL,
            activo INTEGER NOT NULL DEFAULT 1
        );

        CREATE TABLE IF NOT EXISTS usuario (
            id_usuario TEXT PRIMARY KEY,
            username VARCHAR(80) UNIQUE NOT NULL,
            nombre_completo VARCHAR(150) NOT NULL,
            rol VARCHAR(30) NOT NULL,
            password_hash TEXT NOT NULL,
            activo INTEGER NOT NULL DEFAULT 1
        );

        CREATE TABLE IF NOT EXISTS participacion (
            id_proyecto TEXT NOT NULL,
            id_docente TEXT NOT NULL,
            PRIMARY KEY (id_proyecto, id_docente),
            FOREIGN KEY (id_proyecto) REFERENCES proyecto (id_proyecto) ON DELETE CASCADE,
            FOREIGN KEY (id_docente) REFERENCES docente (id_docente) ON DELETE CASCADE
        );
    "#)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn init_mongo(config: &DatabaseConfig) -> Result<Database, AppError> {
    let uri = config.mongodb_uri.as_deref().ok_or_else(|| {
        AppError::ConfigurationError("Falta configurar PJUPI_MONGODB_URI para usar MongoDB.".to_string())
    })?;

    let client = Client::with_uri_str(uri).await?;
    let database = client.database(&config.mongodb_db_name);
    mongo_repo::ensure_indexes(&database).await?;
    Ok(database)
}