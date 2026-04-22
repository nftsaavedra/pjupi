use crate::domain::estadisticas::{DocenteProyectosCount, ExportData, KpisDashboard};
use crate::domain::proyecto::{
    CreateProyectoConParticipantesRequest,
    EliminarProyectoResultado,
    ExportDataConProjectos,
    Proyecto,
    ProyectoDetalle,
    ProyectoParticipanteResumen,
    UpdateProyectoConParticipantesRequest,
};
use crate::error::AppError;
use crate::infrastructure::{mongo_repo, proyecto_repo, sync_outbox_repo};
use crate::services::backend_strategy::get_backend_strategy;
use crate::state::AppState;
use serde_json::json;

fn parse_participantes(detalle: &ProyectoDetalle) -> Result<Vec<ProyectoParticipanteResumen>, AppError> {
    let Some(raw) = detalle.participantes_json.as_deref() else {
        return Ok(Vec::new());
    };

    serde_json::from_str(raw).map_err(|error| {
        AppError::InternalError(format!(
            "No se pudo parsear participantes del proyecto {}: {}",
            detalle.id_proyecto, error
        ))
    })
}

async fn enqueue_proyecto_snapshot(state: &AppState, id_proyecto: &str, operation_type: &str) -> Result<(), AppError> {
    let detalles = proyecto_repo::get_all_proyectos_detalle(state.sqlite_pool()?).await?;
    let detalle = detalles
        .into_iter()
        .find(|item| item.id_proyecto == id_proyecto)
        .ok_or_else(|| AppError::NotFound("Proyecto no encontrado para snapshot offline.".to_string()))?;

    let participantes = parse_participantes(&detalle)?;
    let docentes_ids: Vec<String> = participantes.iter().map(|item| item.id_docente.clone()).collect();
    let docente_responsable_id = participantes
        .iter()
        .find(|item| item.es_responsable)
        .map(|item| item.id_docente.clone());

    let payload = json!({
        "id_proyecto": detalle.id_proyecto,
        "titulo_proyecto": detalle.titulo_proyecto,
        "activo": detalle.activo,
        "docentes_ids": docentes_ids,
        "docente_responsable_id": docente_responsable_id,
    });

    let payload_json = serde_json::to_string(&payload)
        .map_err(|error| AppError::InternalError(format!("No se pudo serializar snapshot offline de proyecto: {}", error)))?;

    sync_outbox_repo::enqueue_operation(
        state.sqlite_pool()?,
        "proyecto",
        id_proyecto,
        operation_type,
        &payload_json,
    )
    .await
}

#[derive(Debug, Clone)]
pub struct ProyectoParticipantesInput {
    pub titulo_proyecto: String,
    pub docentes_ids: Vec<String>,
    pub docente_responsable_id: Option<String>,
}

pub fn prepare_create_input(request: CreateProyectoConParticipantesRequest) -> Result<ProyectoParticipantesInput, AppError> {
    let docentes_ids = normalize_docente_ids(&request.docentes_ids)?;
    if docentes_ids.is_empty() {
        return Err(AppError::InternalError("Seleccione al menos un docente para crear el proyecto.".to_string()));
    }

    let docente_responsable_id = normalize_responsable_id(request.docente_responsable_id);
    validate_responsable(&docentes_ids, &docente_responsable_id)?;

    Ok(ProyectoParticipantesInput {
        titulo_proyecto: request.titulo_proyecto,
        docentes_ids,
        docente_responsable_id,
    })
}

pub fn prepare_update_input(request: UpdateProyectoConParticipantesRequest) -> Result<ProyectoParticipantesInput, AppError> {
    let docentes_ids = normalize_docente_ids(&request.docentes_ids)?;
    let docente_responsable_id = normalize_responsable_id(request.docente_responsable_id);

    validate_responsable(&docentes_ids, &docente_responsable_id)?;

    Ok(ProyectoParticipantesInput {
        titulo_proyecto: request.titulo_proyecto.trim().to_string(),
        docentes_ids,
        docente_responsable_id,
    })
}

pub async fn create(state: &AppState, request: CreateProyectoConParticipantesRequest) -> Result<Proyecto, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        let proyecto = proyecto_repo::create_proyecto_con_participantes(strategy.sqlite()?, request).await?;
        enqueue_proyecto_snapshot(state, &proyecto.id_proyecto, "proyecto.snapshot").await?;
        Ok(proyecto)
    } else {
        mongo_repo::create_proyecto_con_participantes(strategy.mongo()?, request).await
    }
}

pub async fn update(
    state: &AppState,
    id_proyecto: &str,
    request: UpdateProyectoConParticipantesRequest,
) -> Result<Proyecto, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        let proyecto = proyecto_repo::update_proyecto_con_participantes(strategy.sqlite()?, id_proyecto, request).await?;
        enqueue_proyecto_snapshot(state, &proyecto.id_proyecto, "proyecto.snapshot").await?;
        Ok(proyecto)
    } else {
        mongo_repo::update_proyecto_con_participantes(strategy.mongo()?, id_proyecto, request).await
    }
}

pub async fn find_by_docente(state: &AppState, id_docente: &str) -> Result<Vec<Proyecto>, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        proyecto_repo::buscar_proyectos_por_docente(strategy.sqlite()?, id_docente).await
    } else {
        mongo_repo::buscar_proyectos_por_docente(strategy.mongo()?, id_docente).await
    }
}

pub async fn get_all_detalle(state: &AppState) -> Result<Vec<ProyectoDetalle>, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        proyecto_repo::get_all_proyectos_detalle(strategy.sqlite()?).await
    } else {
        mongo_repo::get_all_proyectos_detalle(strategy.mongo()?).await
    }
}

pub async fn delete_relation(state: &AppState, id_proyecto: &str, id_docente: &str) -> Result<(), AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        proyecto_repo::eliminar_relacion_proyecto_docente(strategy.sqlite()?, id_proyecto, id_docente).await?;
        enqueue_proyecto_snapshot(state, id_proyecto, "proyecto.snapshot").await?;
        Ok(())
    } else {
        mongo_repo::eliminar_relacion_proyecto_docente(strategy.mongo()?, id_proyecto, id_docente).await
    }
}

pub async fn delete_relations(state: &AppState, id_proyecto: &str) -> Result<(), AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        proyecto_repo::eliminar_relaciones_proyecto(strategy.sqlite()?, id_proyecto).await?;
        enqueue_proyecto_snapshot(state, id_proyecto, "proyecto.snapshot").await?;
        Ok(())
    } else {
        mongo_repo::eliminar_relaciones_proyecto(strategy.mongo()?, id_proyecto).await
    }
}

pub async fn delete(state: &AppState, id_proyecto: &str) -> Result<EliminarProyectoResultado, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        let result = proyecto_repo::eliminar_proyecto(strategy.sqlite()?, id_proyecto).await?;
        enqueue_proyecto_snapshot(state, id_proyecto, "proyecto.snapshot").await?;
        Ok(result)
    } else {
        mongo_repo::eliminar_proyecto(strategy.mongo()?, id_proyecto).await
    }
}

pub async fn reactivate(state: &AppState, id_proyecto: &str) -> Result<Proyecto, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        let proyecto = proyecto_repo::reactivar_proyecto(strategy.sqlite()?, id_proyecto).await?;
        enqueue_proyecto_snapshot(state, &proyecto.id_proyecto, "proyecto.snapshot").await?;
        Ok(proyecto)
    } else {
        mongo_repo::reactivar_proyecto(strategy.mongo()?, id_proyecto).await
    }
}

pub async fn get_estadisticas_x_docente(state: &AppState) -> Result<Vec<DocenteProyectosCount>, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        proyecto_repo::get_estadisticas_proyectos_x_docente(strategy.sqlite()?).await
    } else {
        mongo_repo::get_estadisticas_proyectos_x_docente(strategy.mongo()?).await
    }
}

pub async fn get_kpis(state: &AppState) -> Result<KpisDashboard, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        proyecto_repo::get_kpis_dashboard(strategy.sqlite()?).await
    } else {
        mongo_repo::get_kpis_dashboard(strategy.mongo()?).await
    }
}

pub async fn get_exportacion_plana(state: &AppState) -> Result<Vec<ExportData>, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        proyecto_repo::get_data_exportacion_plana(strategy.sqlite()?).await
    } else {
        mongo_repo::get_data_exportacion_plana(strategy.mongo()?).await
    }
}

pub async fn get_exportacion_agrupada(state: &AppState) -> Result<Vec<ExportDataConProjectos>, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        proyecto_repo::get_data_exportacion_agrupada_docente(strategy.sqlite()?).await
    } else {
        mongo_repo::get_data_exportacion_agrupada_docente(strategy.mongo()?).await
    }
}

fn normalize_docente_ids(docentes_ids: &[String]) -> Result<Vec<String>, AppError> {
    let mut normalized_ids = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for docente_id in docentes_ids {
        let normalized = docente_id.trim();
        if normalized.is_empty() {
            return Err(AppError::InternalError("La lista de docentes contiene valores inválidos.".to_string()));
        }

        if seen.insert(normalized.to_string()) {
            normalized_ids.push(normalized.to_string());
        }
    }

    Ok(normalized_ids)
}

fn normalize_responsable_id(docente_responsable_id: Option<String>) -> Option<String> {
    docente_responsable_id
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn validate_responsable(docentes_ids: &[String], docente_responsable_id: &Option<String>) -> Result<(), AppError> {
    if docentes_ids.is_empty() {
        if docente_responsable_id.is_some() {
            return Err(AppError::InternalError(
                "No puede asignar un docente responsable cuando el proyecto no tiene docentes vinculados.".to_string(),
            ));
        }
        return Ok(());
    }

    let Some(responsable_id) = docente_responsable_id.as_ref() else {
        return Err(AppError::InternalError(
            "Seleccione un docente responsable para el proyecto.".to_string(),
        ));
    };

    if !docentes_ids.iter().any(|docente_id| docente_id == responsable_id) {
        return Err(AppError::InternalError(
            "El docente responsable debe formar parte de los docentes asignados al proyecto.".to_string(),
        ));
    }

    Ok(())
}