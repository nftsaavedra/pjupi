use mongodb::{Client, Database};
use sqlx::{query, query_as, SqlitePool};

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
            activo INTEGER NOT NULL DEFAULT 1,
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

    // Backward-compatible migration: add 'activo' column if the DB already existed.
    let activo_col_count: (i64,) = query_as(
        "SELECT COUNT(*) FROM pragma_table_info('grado_academico') WHERE name = 'activo'"
    )
    .fetch_one(pool)
    .await?;

    if activo_col_count.0 == 0 {
      query("ALTER TABLE grado_academico ADD COLUMN activo INTEGER NOT NULL DEFAULT 1")
          .execute(pool)
          .await?;
    }

    let docente_activo_col_count: (i64,) = query_as(
        "SELECT COUNT(*) FROM pragma_table_info('docente') WHERE name = 'activo'"
    )
    .fetch_one(pool)
    .await?;

    if docente_activo_col_count.0 == 0 {
      query("ALTER TABLE docente ADD COLUMN activo INTEGER NOT NULL DEFAULT 1")
          .execute(pool)
          .await?;
    }

    let proyecto_activo_col_count: (i64,) = query_as(
        "SELECT COUNT(*) FROM pragma_table_info('proyecto') WHERE name = 'activo'"
    )
    .fetch_one(pool)
    .await?;

    if proyecto_activo_col_count.0 == 0 {
      query("ALTER TABLE proyecto ADD COLUMN activo INTEGER NOT NULL DEFAULT 1")
          .execute(pool)
          .await?;
    }

    let usuario_activo_col_count: (i64,) = query_as(
        "SELECT COUNT(*) FROM pragma_table_info('usuario') WHERE name = 'activo'"
    )
    .fetch_one(pool)
    .await?;

    if usuario_activo_col_count.0 == 0 {
      query("ALTER TABLE usuario ADD COLUMN activo INTEGER NOT NULL DEFAULT 1")
          .execute(pool)
          .await?;
    }

    // Insert default academic grades if not exist
    let count: (i64,) = query_as("SELECT COUNT(*) FROM grado_academico")
        .fetch_one(pool)
        .await?;
    
    if count.0 == 0 {
        query("INSERT INTO grado_academico (id_grado, nombre, descripcion) VALUES (?, ?, ?)")
            .bind(uuid::Uuid::new_v4().to_string())
            .bind("Licenciado")
            .bind("Licenciatura")
            .execute(pool)
            .await?;
        
        query("INSERT INTO grado_academico (id_grado, nombre, descripcion) VALUES (?, ?, ?)")
            .bind(uuid::Uuid::new_v4().to_string())
            .bind("Master")
            .bind("Maestría")
            .execute(pool)
            .await?;
        
        query("INSERT INTO grado_academico (id_grado, nombre, descripcion) VALUES (?, ?, ?)")
            .bind(uuid::Uuid::new_v4().to_string())
            .bind("Doctor")
            .bind("Doctorado")
            .execute(pool)
            .await?;
        
        query("INSERT INTO grado_academico (id_grado, nombre, descripcion) VALUES (?, ?, ?)")
            .bind(uuid::Uuid::new_v4().to_string())
            .bind("Especialista")
            .bind("Especialización")
            .execute(pool)
            .await?;
    }

    Ok(())
}

pub async fn init_mongo(config: &DatabaseConfig) -> Result<Database, AppError> {
    let uri = config.mongodb_uri.as_deref().ok_or_else(|| {
        AppError::ConfigurationError("Falta configurar PJUPI_MONGODB_URI para usar MongoDB.".to_string())
    })?;

    let client = Client::with_uri_str(uri).await?;
    let database = client.database(&config.mongodb_db_name);
    mongo_repo::ensure_indexes_and_seed(&database).await?;
    Ok(database)
}