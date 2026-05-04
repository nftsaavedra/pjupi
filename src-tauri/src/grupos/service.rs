use crate::grupos::models::{GrupoInvestigacion, CreateGrupoInvestigacionRequest, UpdateGrupoInvestigacionRequest};
use crate::grupos::repository;
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use chrono::Utc;

pub async fn get_all(state: &AppState) -> Result<Vec<GrupoInvestigacion>, AppError> {
    let mongo = state.mongo_db()?;
    repository::get_all_grupos(mongo).await
}

pub async fn create(state: &AppState, request: CreateGrupoInvestigacionRequest) -> Result<GrupoInvestigacion, AppError> {
    let mongo = state.mongo_db()?;

    let now_ms = Utc::now().timestamp_millis();
    let mut nuevo_grupo = GrupoInvestigacion::new(request.nombre, now_ms);
    nuevo_grupo.descripcion = request.descripcion;
    nuevo_grupo.coordinador_id = request.coordinador_id;
    nuevo_grupo.lineas_investigacion = request.lineas_investigacion;

    repository::create_grupo(mongo, nuevo_grupo).await
}

pub async fn get_by_id(state: &AppState, id_grupo: &str) -> Result<GrupoInvestigacion, AppError> {
    let mongo = state.mongo_db()?;
    repository::get_grupo_by_id(mongo, id_grupo).await
}

pub async fn update(state: &AppState, id_grupo: &str, request: UpdateGrupoInvestigacionRequest) -> Result<GrupoInvestigacion, AppError> {
    let mongo = state.mongo_db()?;
    repository::update_grupo(mongo, id_grupo, request).await
}

pub async fn delete(state: &AppState, id_grupo: &str) -> Result<(), AppError> {
    let mongo = state.mongo_db()?;
    repository::delete_grupo(mongo, id_grupo).await
}
