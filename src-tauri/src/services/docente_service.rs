use crate::domain::docente::{CreateDocenteRequest, Docente, DocenteDetalle, EliminarDocenteResultado};
use crate::error::AppError;
use crate::infrastructure::{docente_repo, mongo_repo, sync_outbox_repo};
use crate::services::backend_strategy::get_backend_strategy;
use crate::state::AppState;

async fn enqueue_docente_snapshot(state: &AppState, id_docente: &str) -> Result<(), AppError> {
    let docente = docente_repo::get_docente_by_id(state.sqlite_pool()?, id_docente).await?;

    let payload_json = serde_json::to_string(&docente)
        .map_err(|error| AppError::InternalError(format!("No se pudo serializar snapshot offline de docente: {}", error)))?;

    sync_outbox_repo::enqueue_operation(
        state.sqlite_pool()?,
        "docente",
        id_docente,
        "docente.snapshot",
        &payload_json,
    )
    .await
}

pub fn build_delete_result(has_related_projects: bool) -> EliminarDocenteResultado {
    if has_related_projects {
        return EliminarDocenteResultado {
            accion: "desactivado".to_string(),
            mensaje: "Docente desactivado. Mantiene trazabilidad porque tiene proyectos relacionados.".to_string(),
        };
    }

    EliminarDocenteResultado {
        accion: "desactivado".to_string(),
        mensaje: "Docente desactivado correctamente.".to_string(),
    }
}

pub async fn create(state: &AppState, request: CreateDocenteRequest) -> Result<Docente, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        let docente = docente_repo::create_docente(strategy.sqlite()?, request).await?;
        enqueue_docente_snapshot(state, &docente.id_docente).await?;
        Ok(docente)
    } else {
        mongo_repo::create_docente(strategy.mongo()?, request).await
    }
}

pub async fn get_all(state: &AppState) -> Result<Vec<Docente>, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        docente_repo::get_all_docentes(strategy.sqlite()?).await
    } else {
        mongo_repo::get_all_docentes(strategy.mongo()?).await
    }
}

pub async fn find_by_dni(state: &AppState, dni: &str) -> Result<Option<Docente>, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        docente_repo::get_docente_by_dni(strategy.sqlite()?, dni).await
    } else {
        mongo_repo::get_docente_by_dni(strategy.mongo()?, dni).await
    }
}

pub async fn get_all_detalle(state: &AppState) -> Result<Vec<DocenteDetalle>, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        docente_repo::get_all_docentes_con_proyectos(strategy.sqlite()?).await
    } else {
        mongo_repo::get_all_docentes_con_proyectos(strategy.mongo()?).await
    }
}

pub async fn delete(state: &AppState, id_docente: &str) -> Result<EliminarDocenteResultado, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        let result = docente_repo::delete_docente(strategy.sqlite()?, id_docente).await?;
        enqueue_docente_snapshot(state, id_docente).await?;
        Ok(result)
    } else {
        mongo_repo::delete_docente(strategy.mongo()?, id_docente).await
    }
}

pub async fn reactivate(state: &AppState, id_docente: &str) -> Result<Docente, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        let docente = docente_repo::reactivar_docente(strategy.sqlite()?, id_docente).await?;
        enqueue_docente_snapshot(state, &docente.id_docente).await?;
        Ok(docente)
    } else {
        mongo_repo::reactivar_docente(strategy.mongo()?, id_docente).await
    }
}

pub async fn get_by_id(state: &AppState, id_docente: &str) -> Result<Docente, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        docente_repo::get_docente_by_id(strategy.sqlite()?, id_docente).await
    } else {
        mongo_repo::get_docente_by_id(strategy.mongo()?, id_docente).await
    }
}

pub async fn update_renacyt(state: &AppState, docente: &Docente) -> Result<(), AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        docente_repo::update_docente_renacyt(strategy.sqlite()?, docente).await?;
        enqueue_docente_snapshot(state, &docente.id_docente).await?;
        Ok(())
    } else {
        mongo_repo::update_docente_renacyt(strategy.mongo()?, docente).await
    }
}

pub async fn get_detalle_by_id(state: &AppState, id_docente: &str) -> Result<DocenteDetalle, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        docente_repo::get_docente_detalle_by_id(strategy.sqlite()?, id_docente).await
    } else {
        mongo_repo::get_docente_detalle_by_id(strategy.mongo()?, id_docente).await
    }
}