use tauri::{State, Window};

use crate::catalogos::models::{CatalogoItem, CreateCatalogoRequest, EliminarCatalogoResultado};
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use crate::shared::access_control;

#[tauri::command]
pub async fn get_catalogos(
    window: Window,
    state: State<'_, AppState>,
    tipo: String,
) -> Result<Vec<CatalogoItem>, AppError> {
    access_control::get_catalogos(&state, window.label(), &tipo).await
}

#[tauri::command]
pub async fn get_all_catalogos_admin(
    window: Window,
    state: State<'_, AppState>,
    tipo: String,
) -> Result<Vec<CatalogoItem>, AppError> {
    access_control::get_all_catalogos_admin(&state, window.label(), &tipo).await
}

#[tauri::command]
pub async fn crear_catalogo(
    window: Window,
    state: State<'_, AppState>,
    request: CreateCatalogoRequest,
) -> Result<CatalogoItem, AppError> {
    access_control::crear_catalogo(&state, window.label(), request).await
}

#[tauri::command]
pub async fn actualizar_catalogo(
    window: Window,
    state: State<'_, AppState>,
    id: String,
    request: CreateCatalogoRequest,
) -> Result<CatalogoItem, AppError> {
    access_control::actualizar_catalogo(&state, window.label(), &id, request).await
}

#[tauri::command]
pub async fn eliminar_catalogo(
    window: Window,
    state: State<'_, AppState>,
    id: String,
) -> Result<EliminarCatalogoResultado, AppError> {
    access_control::eliminar_catalogo(&state, window.label(), &id).await
}

#[tauri::command]
pub async fn reactivar_catalogo(
    window: Window,
    state: State<'_, AppState>,
    id: String,
) -> Result<CatalogoItem, AppError> {
    access_control::reactivar_catalogo(&state, window.label(), &id).await
}
