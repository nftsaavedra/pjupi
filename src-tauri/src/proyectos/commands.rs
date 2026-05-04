use tauri::{State, Window};
use crate::proyectos::models::{
    Proyecto,
    CreateProyectoConParticipantesRequest,
    UpdateProyectoConParticipantesRequest,
    ProyectoDetalle,
    EliminarProyectoResultado,
};
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use crate::shared::access_control;

#[tauri::command]
pub async fn crear_proyecto_con_participantes(
    window: Window,
    state: State<'_, AppState>,
    request: CreateProyectoConParticipantesRequest,
) -> Result<Proyecto, AppError> {
    access_control::crear_proyecto_con_participantes(&state, window.label(), request).await
}

#[tauri::command]
pub async fn buscar_proyectos_por_docente(
    window: Window,
    state: State<'_, AppState>,
    id_docente: String,
) -> Result<Vec<Proyecto>, AppError> {
    access_control::buscar_proyectos_por_docente(&state, window.label(), &id_docente).await
}

#[tauri::command]
pub async fn actualizar_proyecto_con_participantes(
    window: Window,
    state: State<'_, AppState>,
    id_proyecto: String,
    request: UpdateProyectoConParticipantesRequest,
) -> Result<Proyecto, AppError> {
    access_control::update_proyecto_con_participantes(&state, window.label(), &id_proyecto, request).await
}

#[tauri::command]
pub async fn get_all_proyectos_detalle(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Vec<ProyectoDetalle>, AppError> {
    access_control::get_all_proyectos_detalle(&state, window.label()).await
}

#[tauri::command]
pub async fn eliminar_relacion_proyecto_docente(
    window: Window,
    state: State<'_, AppState>,
    id_proyecto: String,
    id_docente: String,
) -> Result<(), AppError> {
    access_control::eliminar_relacion_proyecto_docente(&state, window.label(), &id_proyecto, &id_docente).await
}

#[tauri::command]
pub async fn eliminar_relaciones_proyecto(
    window: Window,
    state: State<'_, AppState>,
    id_proyecto: String,
) -> Result<(), AppError> {
    access_control::eliminar_relaciones_proyecto(&state, window.label(), &id_proyecto).await
}

#[tauri::command]
pub async fn eliminar_proyecto(
    window: Window,
    state: State<'_, AppState>,
    id_proyecto: String,
) -> Result<EliminarProyectoResultado, AppError> {
    access_control::eliminar_proyecto(&state, window.label(), &id_proyecto).await
}

#[tauri::command]
pub async fn reactivar_proyecto(
    window: Window,
    state: State<'_, AppState>,
    id_proyecto: String,
) -> Result<Proyecto, AppError> {
    access_control::reactivar_proyecto(&state, window.label(), &id_proyecto).await
}
