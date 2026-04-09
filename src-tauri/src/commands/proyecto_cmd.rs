use tauri::State;
use crate::domain::proyecto::{
    Proyecto,
    CreateProyectoConParticipantesRequest,
    ProyectoDetalle,
    EliminarProyectoResultado,
};
use crate::error::AppError;
use crate::state::AppState;
use crate::storage;

#[tauri::command]
pub async fn crear_proyecto_con_participantes(
    state: State<'_, AppState>,
    actor_user_id: String,
    request: CreateProyectoConParticipantesRequest,
) -> Result<Proyecto, AppError> {
    storage::crear_proyecto_con_participantes(&state, &actor_user_id, request).await
}

#[tauri::command]
pub async fn buscar_proyectos_por_docente(
    state: State<'_, AppState>,
    actor_user_id: String,
    id_docente: String,
) -> Result<Vec<Proyecto>, AppError> {
    storage::buscar_proyectos_por_docente(&state, &actor_user_id, &id_docente).await
}

#[tauri::command]
pub async fn get_all_proyectos_detalle(
    state: State<'_, AppState>,
    actor_user_id: String,
) -> Result<Vec<ProyectoDetalle>, AppError> {
    storage::get_all_proyectos_detalle(&state, &actor_user_id).await
}

#[tauri::command]
pub async fn eliminar_relacion_proyecto_docente(
    state: State<'_, AppState>,
    actor_user_id: String,
    id_proyecto: String,
    id_docente: String,
) -> Result<(), AppError> {
    storage::eliminar_relacion_proyecto_docente(&state, &actor_user_id, &id_proyecto, &id_docente).await
}

#[tauri::command]
pub async fn eliminar_relaciones_proyecto(
    state: State<'_, AppState>,
    actor_user_id: String,
    id_proyecto: String,
) -> Result<(), AppError> {
    storage::eliminar_relaciones_proyecto(&state, &actor_user_id, &id_proyecto).await
}

#[tauri::command]
pub async fn eliminar_proyecto(
    state: State<'_, AppState>,
    actor_user_id: String,
    id_proyecto: String,
) -> Result<EliminarProyectoResultado, AppError> {
    storage::eliminar_proyecto(&state, &actor_user_id, &id_proyecto).await
}

#[tauri::command]
pub async fn reactivar_proyecto(
    state: State<'_, AppState>,
    actor_user_id: String,
    id_proyecto: String,
) -> Result<Proyecto, AppError> {
    storage::reactivar_proyecto(&state, &actor_user_id, &id_proyecto).await
}