use crate::domain::grado::{CreateGradoRequest, EliminarGradoResultado, GradoAcademico};
use crate::error::AppError;
use crate::infrastructure::{grado_repo, mongo_repo, sync_outbox_repo};
use crate::services::backend_strategy::get_backend_strategy;
use crate::state::AppState;
use serde_json::json;

async fn enqueue_grado_outbox(
    state: &AppState,
    aggregate_id: &str,
    operation_type: &str,
    payload: serde_json::Value,
) -> Result<(), AppError> {
    let payload_json = serde_json::to_string(&payload)
        .map_err(|error| AppError::InternalError(format!("No se pudo serializar el evento offline de grado: {}", error)))?;

    sync_outbox_repo::enqueue_operation(
        state.sqlite_pool()?,
        "grado",
        aggregate_id,
        operation_type,
        &payload_json,
    )
    .await
}

pub async fn get_all(state: &AppState) -> Result<Vec<GradoAcademico>, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        grado_repo::get_all_grados(strategy.sqlite()?).await
    } else {
        mongo_repo::get_all_grados(strategy.mongo()?).await
    }
}

pub async fn create(state: &AppState, request: CreateGradoRequest) -> Result<GradoAcademico, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        let grado = grado_repo::create_grado(strategy.sqlite()?, request).await?;
        enqueue_grado_outbox(
            state,
            &grado.id_grado,
            "grado.create",
            json!({
                "id_grado": grado.id_grado,
                "nombre": grado.nombre,
                "descripcion": grado.descripcion,
                "activo": grado.activo,
            }),
        )
        .await?;
        Ok(grado)
    } else {
        mongo_repo::create_grado(strategy.mongo()?, request).await
    }
}

pub async fn update(state: &AppState, id_grado: &str, request: CreateGradoRequest) -> Result<GradoAcademico, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        let grado = grado_repo::update_grado(strategy.sqlite()?, id_grado, request).await?;
        enqueue_grado_outbox(
            state,
            &grado.id_grado,
            "grado.update",
            json!({
                "id_grado": grado.id_grado,
                "nombre": grado.nombre,
                "descripcion": grado.descripcion,
                "activo": grado.activo,
            }),
        )
        .await?;
        Ok(grado)
    } else {
        mongo_repo::update_grado(strategy.mongo()?, id_grado, request).await
    }
}

pub async fn delete(state: &AppState, id_grado: &str) -> Result<EliminarGradoResultado, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        let result = grado_repo::delete_grado(strategy.sqlite()?, id_grado).await?;
        enqueue_grado_outbox(
            state,
            id_grado,
            "grado.delete",
            json!({
                "id_grado": id_grado,
                "accion": result.accion,
                "mensaje": result.mensaje,
            }),
        )
        .await?;
        Ok(result)
    } else {
        mongo_repo::delete_grado(strategy.mongo()?, id_grado).await
    }
}

pub async fn reactivate(state: &AppState, id_grado: &str) -> Result<GradoAcademico, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        let grado = grado_repo::reactivar_grado(strategy.sqlite()?, id_grado).await?;
        enqueue_grado_outbox(
            state,
            &grado.id_grado,
            "grado.reactivate",
            json!({
                "id_grado": grado.id_grado,
                "nombre": grado.nombre,
                "descripcion": grado.descripcion,
                "activo": grado.activo,
            }),
        )
        .await?;
        Ok(grado)
    } else {
        mongo_repo::reactivar_grado(strategy.mongo()?, id_grado).await
    }
}