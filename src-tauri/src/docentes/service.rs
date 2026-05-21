use crate::docentes::models::{CreateDocenteRequest, Docente, DocenteDetalle, EliminarDocenteResultado, RefreshDocenteRenacytFormacionResultado};
use crate::shared::error::AppError;
use crate::shared::pagination::PaginatedResult;
use crate::docentes::repository;
use crate::shared::state::AppState;

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
    let db = state.mongo_db()?;
    repository::create_docente(db, request).await
}

pub async fn get_all(state: &AppState) -> Result<Vec<Docente>, AppError> {
    let db = state.mongo_db()?;
    repository::get_all_docentes(db).await
}

pub async fn get_all_paginated(state: &AppState, page: u32, limit: u32) -> Result<PaginatedResult<Docente>, AppError> {
    let db = state.mongo_db()?;
    repository::get_all_docentes_paginated(db, page, limit).await
}

pub async fn find_by_dni(state: &AppState, dni: &str) -> Result<Option<Docente>, AppError> {
    let db = state.mongo_db()?;
    repository::get_docente_by_dni(db, dni).await
}

pub async fn get_all_detalle(state: &AppState) -> Result<Vec<DocenteDetalle>, AppError> {
    let db = state.mongo_db()?;
    repository::get_all_docentes_con_proyectos(db).await
}

pub async fn delete(state: &AppState, id_docente: &str) -> Result<EliminarDocenteResultado, AppError> {
    let db = state.mongo_db()?;
    repository::delete_docente(db, id_docente).await
}

pub async fn reactivate(state: &AppState, id_docente: &str) -> Result<Docente, AppError> {
    let db = state.mongo_db()?;
    repository::reactivar_docente(db, id_docente).await
}

pub async fn get_by_id(state: &AppState, id_docente: &str) -> Result<Docente, AppError> {
    let db = state.mongo_db()?;
    repository::get_docente_by_id(db, id_docente).await
}

pub async fn update_renacyt(state: &AppState, docente: &Docente) -> Result<(), AppError> {
    let db = state.mongo_db()?;
    repository::update_docente_renacyt(db, docente).await
}

pub async fn get_detalle_by_id(state: &AppState, id_docente: &str) -> Result<DocenteDetalle, AppError> {
    let db = state.mongo_db()?;
    repository::get_docente_detalle_by_id(db, id_docente).await
}

pub async fn refresh_renacyt_formacion(
    state: &AppState,
    id_docente: &str,
) -> Result<RefreshDocenteRenacytFormacionResultado, AppError> {
    let mut docente = get_by_id(state, id_docente).await?;
    let codigo_o_id = docente
        .renacyt_id_investigador
        .clone()
        .or_else(|| docente.renacyt_codigo_registro.clone())
        .ok_or_else(|| AppError::ExternalServiceError("El docente no tiene un vínculo RENACYT para refrescar su formación académica.".to_string()))?;
    let tenia_formaciones = docente
        .renacyt_formaciones_academicas_json
        .as_ref()
        .is_some_and(|value| !value.trim().is_empty());
    let lookup = crate::shared::external::renacyt_client::consultar_investigador(state.renacyt_config(), &codigo_o_id).await?;
    let actualizada = docente.apply_renacyt_refresh(lookup);
    update_renacyt(state, &docente).await?;
    let docente_detalle = get_detalle_by_id(state, id_docente).await?;
    let mensaje = if actualizada {
        "Formación académica RENACYT actualizada correctamente.".to_string()
    } else if tenia_formaciones {
        "RENACYT no devolvió nueva formación académica en esta sincronización. Se mantuvo la información registrada.".to_string()
    } else {
        "RENACYT no devolvió formación académica disponible para este docente en esta sincronización.".to_string()
    };
    Ok(RefreshDocenteRenacytFormacionResultado {
        docente: docente_detalle,
        actualizada,
        mensaje,
    })
}
