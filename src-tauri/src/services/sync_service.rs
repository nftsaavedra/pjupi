use mongodb::{
    bson::{doc, Document},
    options::ReplaceOptions,
    Database,
};
use sqlx::SqlitePool;

use crate::domain::docente::Docente;
use crate::domain::grado::GradoAcademico;
use crate::domain::usuario::Usuario;
use crate::error::AppError;
use crate::infrastructure::sync_outbox_repo;

pub async fn sync_pending_once(sqlite_pool: &SqlitePool, mongo_db: &Database) -> Result<(), AppError> {
    let pending = sync_outbox_repo::get_pending_operations(sqlite_pool, 100).await?;

    for operation in pending {
        let result = process_operation(sqlite_pool, mongo_db, &operation).await;

        match result {
            Ok(()) => {
                sync_outbox_repo::mark_operation_completed(sqlite_pool, operation.id).await?;
                log_sync_operation(&operation, "success", None);
            },
            Err(error) => {
                let failure_message = format!("[attempt {}] {}", operation.retry_count + 1, error);
                sync_outbox_repo::mark_operation_failed(sqlite_pool, operation.id, &failure_message).await?;
                log_sync_operation(&operation, "failed", Some(&error.to_string()));
            }
        }
    }

    Ok(())
}

fn log_sync_operation(operation: &sync_outbox_repo::SyncOutboxItem, status: &str, error: Option<&str>) {
    let timestamp_ms = operation.created_at;
    let status_label = match status {
        "success" => "✓ SYNCED",
        "failed" => "✗ FAILED",
        _ => "• PROCESSED",
    };
    
    if let Some(err) = error {
        eprintln!(
            "[sync] {} {}:{}::{} at {} - Error: {}",
            status_label,
            operation.aggregate_type,
            operation.aggregate_id,
            operation.operation_type,
            timestamp_ms,
            err
        );
    } else {
        eprintln!(
            "[sync] {} {}:{}::{} at {}",
            status_label,
            operation.aggregate_type,
            operation.aggregate_id,
            operation.operation_type,
            timestamp_ms,
        );
    }
}

async fn process_operation(
    sqlite_pool: &SqlitePool,
    mongo_db: &Database,
    operation: &sync_outbox_repo::SyncOutboxItem,
) -> Result<(), AppError> {
    match (operation.aggregate_type.as_str(), operation.operation_type.as_str()) {
        ("grado", "grado.create") | ("grado", "grado.update") | ("grado", "grado.reactivate") => {
            let grado: GradoAcademico = serde_json::from_str(&operation.payload_json)
                .map_err(|error| AppError::InternalError(format!("Payload offline inválido para grado: {}", error)))?;

            check_and_log_conflict(sqlite_pool, mongo_db, "grados", &operation.aggregate_id, operation.created_at).await.ok();

            let options = ReplaceOptions::builder().upsert(true).build();
            mongo_db
                .collection::<GradoAcademico>("grados")
                .replace_one(doc! { "id_grado": &grado.id_grado }, &grado)
                .with_options(options)
                .await?;

            Ok(())
        }
        ("grado", "grado.delete") => {
            let payload: Document = serde_json::from_str(&operation.payload_json)
                .map_err(|error| AppError::InternalError(format!("Payload offline inválido para eliminar grado: {}", error)))?;

            let accion = payload.get_str("accion").unwrap_or("desactivado");
            let id_grado = payload.get_str("id_grado").unwrap_or(operation.aggregate_id.as_str());

            if accion == "eliminado" {
                mongo_db
                    .collection::<Document>("grados")
                    .delete_one(doc! { "id_grado": id_grado })
                    .await?;
            } else {
                mongo_db
                    .collection::<Document>("grados")
                    .update_one(doc! { "id_grado": id_grado }, doc! { "$set": { "activo": 0i64 } })
                    .await?;
            }

            Ok(())
        }
        ("proyecto", "proyecto.snapshot") => {
            let payload: Document = serde_json::from_str(&operation.payload_json)
                .map_err(|error| AppError::InternalError(format!("Payload offline inválido para proyecto: {}", error)))?;

            let id_proyecto = payload
                .get_str("id_proyecto")
                .map_err(|_| AppError::InternalError("Snapshot offline de proyecto sin id_proyecto.".to_string()))?;
            
            check_and_log_conflict(sqlite_pool, mongo_db, "proyectos", id_proyecto, operation.created_at).await.ok();

            let titulo_proyecto = payload
                .get_str("titulo_proyecto")
                .map_err(|_| AppError::InternalError("Snapshot offline de proyecto sin titulo_proyecto.".to_string()))?;
            let activo = payload.get_i64("activo").unwrap_or(1);

            mongo_db
                .collection::<Document>("proyectos")
                .update_one(
                    doc! { "id_proyecto": id_proyecto },
                    doc! {
                        "$set": {
                            "id_proyecto": id_proyecto,
                            "titulo_proyecto": titulo_proyecto,
                            "activo": activo,
                        }
                    },
                )
                .upsert(true)
                .await?;

            mongo_db
                .collection::<Document>("participaciones")
                .delete_many(doc! { "id_proyecto": id_proyecto })
                .await?;

            let responsable_id = payload.get_str("docente_responsable_id").ok().map(str::to_string);
            if let Some(docentes_bson) = payload.get("docentes_ids") {
                let docentes = docentes_bson.as_array().cloned().unwrap_or_default();
                for docente in docentes {
                    let Some(id_docente) = docente.as_str() else {
                        continue;
                    };

                    let es_responsable = responsable_id
                        .as_deref()
                        .is_some_and(|responsable| responsable == id_docente);

                    mongo_db
                        .collection::<Document>("participaciones")
                        .insert_one(doc! {
                            "_id": format!("{}:{}", id_proyecto, id_docente),
                            "id_proyecto": id_proyecto,
                            "id_docente": id_docente,
                            "es_responsable": es_responsable,
                        })
                        .await?;
                }
            }

            Ok(())
        }
        ("docente", "docente.snapshot") => {
            let docente: Docente = serde_json::from_str(&operation.payload_json)
                .map_err(|error| AppError::InternalError(format!("Payload offline inválido para docente: {}", error)))?;

            check_and_log_conflict(sqlite_pool, mongo_db, "docentes", &operation.aggregate_id, operation.created_at).await.ok();

            let options = ReplaceOptions::builder().upsert(true).build();
            mongo_db
                .collection::<Docente>("docentes")
                .replace_one(doc! { "id_docente": &docente.id_docente }, &docente)
                .with_options(options)
                .await?;

            Ok(())
        }
        ("usuario", "usuario.snapshot") => {
            let usuario: Usuario = serde_json::from_str(&operation.payload_json)
                .map_err(|error| AppError::InternalError(format!("Payload offline inválido para usuario: {}", error)))?;

            check_and_log_conflict(sqlite_pool, mongo_db, "usuarios", &operation.aggregate_id, operation.created_at).await.ok();

            let options = ReplaceOptions::builder().upsert(true).build();
            mongo_db
                .collection::<Usuario>("usuarios")
                .replace_one(doc! { "id_usuario": &usuario.id_usuario }, &usuario)
                .with_options(options)
                .await?;

            Ok(())
        }
        _ => Err(AppError::InternalError(format!(
            "Operación offline no soportada aún: {}:{}",
            operation.aggregate_type, operation.operation_type
        ))),
    }
}

async fn check_and_log_conflict(
    sqlite_pool: &SqlitePool,
    mongo_db: &Database,
    collection_name: &str,
    aggregate_id: &str,
    offline_created_at: i64,
) -> Result<(), AppError> {
    let id_field = match collection_name {
        "grados" => "id_grado",
        "proyectos" => "id_proyecto",
        "docentes" => "id_docente",
        "usuarios" => "id_usuario",
        _ => "id",
    };

    let existing = mongo_db
        .collection::<Document>(collection_name)
        .find_one(doc! { id_field: aggregate_id })
        .await?;

    if let Some(existing_doc) = existing {
        let mongo_updated_at = existing_doc
            .get("updated_at")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        
        let mut conflict_type = "update_conflict";
        let mut description = format!(
            "MongoDB document already exists for {}:{}. Offline updated at: {}, MongoDB updated at: {}",
            collection_name, aggregate_id, offline_created_at, mongo_updated_at
        );

        if offline_created_at > mongo_updated_at {
            conflict_type = "newer_offline";
            description.push_str(" (offline version is newer, but MongoDB-primary policy will preserve MongoDB version)");
        } else if offline_created_at < mongo_updated_at {
            description.push_str(" (MongoDB version is newer, correctly preserved)");
        } else {
            description.push_str(" (both have same timestamp)");
        }

        log_conflict_to_db(
            sqlite_pool,
            collection_name,
            aggregate_id,
            Some(offline_created_at),
            Some(mongo_updated_at),
            conflict_type,
            &description,
            "mongodb_primary_applied",
        )
        .await
        .ok();

        eprintln!(
            "[sync-conflict] {} (policy: MongoDB-primary) - {}",
            collection_name, description
        );
    }

    Ok(())
}

async fn log_conflict_to_db(
    sqlite_pool: &SqlitePool,
    aggregate_type: &str,
    aggregate_id: &str,
    offline_updated_at: Option<i64>,
    mongo_updated_at: Option<i64>,
    conflict_type: &str,
    description: &str,
    resolution: &str,
) -> Result<(), AppError> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    sqlx::query(
        r#"
        INSERT INTO sync_conflicts 
        (aggregate_type, aggregate_id, offline_updated_at, mongo_updated_at, conflict_type, description, resolution, created_at, resolved_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(aggregate_type)
    .bind(aggregate_id)
    .bind(offline_updated_at)
    .bind(mongo_updated_at)
    .bind(conflict_type)
    .bind(description)
    .bind(resolution)
    .bind(now)
    .bind(now)
    .execute(sqlite_pool)
    .await?;

    Ok(())
}