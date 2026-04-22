use mongodb::{Client, Database};
use sqlx::{query, Row, SqlitePool};

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
            es_responsable INTEGER NOT NULL DEFAULT 0,
            PRIMARY KEY (id_proyecto, id_docente),
            FOREIGN KEY (id_proyecto) REFERENCES proyecto (id_proyecto) ON DELETE CASCADE,
            FOREIGN KEY (id_docente) REFERENCES docente (id_docente) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS sync_outbox (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            aggregate_type TEXT NOT NULL,
            aggregate_id TEXT NOT NULL,
            operation_type TEXT NOT NULL,
            payload_json TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            retry_count INTEGER NOT NULL DEFAULT 0,
            last_error TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS sync_state (
            scope TEXT PRIMARY KEY,
            last_synced_at INTEGER,
            last_synced_version TEXT,
            metadata_json TEXT
        );

        CREATE TABLE IF NOT EXISTS sync_conflicts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            aggregate_type TEXT NOT NULL,
            aggregate_id TEXT NOT NULL,
            offline_updated_at INTEGER,
            mongo_updated_at INTEGER,
            conflict_type TEXT NOT NULL,
            description TEXT,
            resolution TEXT,
            created_at INTEGER NOT NULL,
            resolved_at INTEGER
        );
    "#)
    .execute(pool)
    .await?;

    let participacion_columns = query("PRAGMA table_info(participacion)")
        .fetch_all(pool)
        .await?;
    let has_es_responsable = participacion_columns
        .iter()
        .any(|column| column.try_get::<String, _>("name").map(|name| name == "es_responsable").unwrap_or(false));

    if !has_es_responsable {
        query("ALTER TABLE participacion ADD COLUMN es_responsable INTEGER NOT NULL DEFAULT 0")
            .execute(pool)
            .await?;
    }

    query("CREATE INDEX IF NOT EXISTS idx_sync_outbox_status_created_at ON sync_outbox(status, created_at)")
        .execute(pool)
        .await?;
    query("CREATE INDEX IF NOT EXISTS idx_sync_outbox_aggregate ON sync_outbox(aggregate_type, aggregate_id)")
        .execute(pool)
        .await?;
    query("CREATE INDEX IF NOT EXISTS idx_sync_conflicts_unresolved ON sync_conflicts(resolved_at) WHERE resolved_at IS NULL")
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