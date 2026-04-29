use crate::domain::estadisticas::{DocenteProyectosCount, ExportData, KpisDashboard};
use crate::domain::proyecto::{
    CreateProyectoConParticipantesRequest,
    EliminarProyectoResultado,
    ExportDataConProjectos,
    Proyecto,
    ProyectoDetalle,
    UpdateProyectoConParticipantesRequest,
};
use crate::error::AppError;
use crate::infrastructure::mongo_repo;
use crate::state::AppState;

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
    let db = state.mongo_db()?;
    mongo_repo::create_proyecto_con_participantes(db, request).await
}

pub async fn update(
    state: &AppState,
    id_proyecto: &str,
    request: UpdateProyectoConParticipantesRequest,
) -> Result<Proyecto, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::update_proyecto_con_participantes(db, id_proyecto, request).await
}

pub async fn find_by_docente(state: &AppState, id_docente: &str) -> Result<Vec<Proyecto>, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::buscar_proyectos_por_docente(db, id_docente).await
}

pub async fn get_all_detalle(state: &AppState) -> Result<Vec<ProyectoDetalle>, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_all_proyectos_detalle(db).await
}

pub async fn delete_relation(state: &AppState, id_proyecto: &str, id_docente: &str) -> Result<(), AppError> {
    let db = state.mongo_db()?;
    mongo_repo::eliminar_relacion_proyecto_docente(db, id_proyecto, id_docente).await
}

pub async fn delete_relations(state: &AppState, id_proyecto: &str) -> Result<(), AppError> {
    let db = state.mongo_db()?;
    mongo_repo::eliminar_relaciones_proyecto(db, id_proyecto).await
}

pub async fn delete(state: &AppState, id_proyecto: &str) -> Result<EliminarProyectoResultado, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::eliminar_proyecto(db, id_proyecto).await
}

pub async fn reactivate(state: &AppState, id_proyecto: &str) -> Result<Proyecto, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::reactivar_proyecto(db, id_proyecto).await
}

pub async fn get_estadisticas_x_docente(state: &AppState) -> Result<Vec<DocenteProyectosCount>, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_estadisticas_proyectos_x_docente(db).await
}

pub async fn get_kpis(state: &AppState) -> Result<KpisDashboard, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_kpis_dashboard(db).await
}

pub async fn get_exportacion_plana(state: &AppState) -> Result<Vec<ExportData>, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_data_exportacion_plana(db).await
}

pub async fn get_exportacion_agrupada(state: &AppState) -> Result<Vec<ExportDataConProjectos>, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_data_exportacion_agrupada_docente(db).await
}

fn normalize_docente_ids(docentes_ids: &[String]) -> Result<Vec<String>, AppError> {
    let mut normalized_ids = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for docente_id in docentes_ids {
        let normalized = docente_id.trim();
        if normalized.is_empty() {
            return Err(AppError::InternalError("La lista de docentes contiene valores invalidos.".to_string()));
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
