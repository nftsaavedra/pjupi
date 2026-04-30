use tauri::{State, Window};
use crate::domain::patente::{CreatePatenteRequest, Patente, UpdatePatenteRequest};
use crate::domain::producto::{CreateProductoRequest, Producto, UpdateProductoRequest};
use crate::domain::equipamiento::{CreateEquipamientoRequest, Equipamiento, UpdateEquipamientoRequest};
use crate::domain::financiamiento::{CreateFinanciamientoRequest, Financiamiento, UpdateFinanciamientoRequest};
use crate::error::AppError;
use crate::state::AppState;
use crate::storage;

// ── Patentes ──────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn crear_patente(
    window: Window,
    state: State<'_, AppState>,
    request: CreatePatenteRequest,
) -> Result<Patente, AppError> {
    storage::crear_patente(&state, window.label(), request).await
}

#[tauri::command]
pub async fn get_patentes_proyecto(
    window: Window,
    state: State<'_, AppState>,
    proyecto_id: String,
) -> Result<Vec<Patente>, AppError> {
    storage::get_patentes_proyecto(&state, window.label(), &proyecto_id).await
}

#[tauri::command]
pub async fn actualizar_patente(
    window: Window,
    state: State<'_, AppState>,
    id_patente: String,
    request: UpdatePatenteRequest,
) -> Result<Patente, AppError> {
    storage::actualizar_patente(&state, window.label(), &id_patente, request).await
}

#[tauri::command]
pub async fn eliminar_patente(
    window: Window,
    state: State<'_, AppState>,
    id_patente: String,
) -> Result<(), AppError> {
    storage::eliminar_patente(&state, window.label(), &id_patente).await
}

// ── Productos ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn crear_producto(
    window: Window,
    state: State<'_, AppState>,
    request: CreateProductoRequest,
) -> Result<Producto, AppError> {
    storage::crear_producto(&state, window.label(), request).await
}

#[tauri::command]
pub async fn get_productos_proyecto(
    window: Window,
    state: State<'_, AppState>,
    proyecto_id: String,
) -> Result<Vec<Producto>, AppError> {
    storage::get_productos_proyecto(&state, window.label(), &proyecto_id).await
}

#[tauri::command]
pub async fn actualizar_producto(
    window: Window,
    state: State<'_, AppState>,
    id_producto: String,
    request: UpdateProductoRequest,
) -> Result<Producto, AppError> {
    storage::actualizar_producto(&state, window.label(), &id_producto, request).await
}

#[tauri::command]
pub async fn eliminar_producto(
    window: Window,
    state: State<'_, AppState>,
    id_producto: String,
) -> Result<(), AppError> {
    storage::eliminar_producto(&state, window.label(), &id_producto).await
}

// ── Equipamientos ─────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn crear_equipamiento(
    window: Window,
    state: State<'_, AppState>,
    request: CreateEquipamientoRequest,
) -> Result<Equipamiento, AppError> {
    storage::crear_equipamiento(&state, window.label(), request).await
}

#[tauri::command]
pub async fn get_equipamientos_proyecto(
    window: Window,
    state: State<'_, AppState>,
    proyecto_id: String,
) -> Result<Vec<Equipamiento>, AppError> {
    storage::get_equipamientos_proyecto(&state, window.label(), &proyecto_id).await
}

#[tauri::command]
pub async fn actualizar_equipamiento(
    window: Window,
    state: State<'_, AppState>,
    id_equipamiento: String,
    request: UpdateEquipamientoRequest,
) -> Result<Equipamiento, AppError> {
    storage::actualizar_equipamiento(&state, window.label(), &id_equipamiento, request).await
}

#[tauri::command]
pub async fn eliminar_equipamiento(
    window: Window,
    state: State<'_, AppState>,
    id_equipamiento: String,
) -> Result<(), AppError> {
    storage::eliminar_equipamiento(&state, window.label(), &id_equipamiento).await
}

// ── Financiamientos ───────────────────────────────────────────────────────────

#[tauri::command]
pub async fn crear_financiamiento(
    window: Window,
    state: State<'_, AppState>,
    request: CreateFinanciamientoRequest,
) -> Result<Financiamiento, AppError> {
    storage::crear_financiamiento(&state, window.label(), request).await
}

#[tauri::command]
pub async fn get_financiamientos_proyecto(
    window: Window,
    state: State<'_, AppState>,
    proyecto_id: String,
) -> Result<Vec<Financiamiento>, AppError> {
    storage::get_financiamientos_proyecto(&state, window.label(), &proyecto_id).await
}

#[tauri::command]
pub async fn actualizar_financiamiento(
    window: Window,
    state: State<'_, AppState>,
    id_financiamiento: String,
    request: UpdateFinanciamientoRequest,
) -> Result<Financiamiento, AppError> {
    storage::actualizar_financiamiento(&state, window.label(), &id_financiamiento, request).await
}

#[tauri::command]
pub async fn eliminar_financiamiento(
    window: Window,
    state: State<'_, AppState>,
    id_financiamiento: String,
) -> Result<(), AppError> {
    storage::eliminar_financiamiento(&state, window.label(), &id_financiamiento).await
}
