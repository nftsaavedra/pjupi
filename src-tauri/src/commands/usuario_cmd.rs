use tauri::{State, Window};

use crate::domain::usuario::{AuthStatus, BootstrapUsuarioRequest, CreateUsuarioRequest, LoginUsuarioRequest, UpdateUsuarioRequest, Usuario};
use crate::error::AppError;
use crate::state::AppState;
use crate::storage;

#[tauri::command]
pub async fn crear_usuario(
    window: Window,
    state: State<'_, AppState>,
    request: CreateUsuarioRequest,
) -> Result<Usuario, AppError> {
    storage::crear_usuario(&state, window.label(), request).await
}

#[tauri::command]
pub async fn get_auth_status(
    state: State<'_, AppState>,
) -> Result<AuthStatus, AppError> {
    storage::get_auth_status(&state).await
}

#[tauri::command]
pub async fn registrar_primer_usuario(
    window: Window,
    state: State<'_, AppState>,
    request: BootstrapUsuarioRequest,
) -> Result<Usuario, AppError> {
    storage::registrar_primer_usuario(&state, window.label(), request).await
}

#[tauri::command]
pub async fn login_usuario(
    window: Window,
    state: State<'_, AppState>,
    request: LoginUsuarioRequest,
) -> Result<Usuario, AppError> {
    storage::login_usuario(&state, window.label(), request).await
}

#[tauri::command]
pub async fn get_current_session(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Option<Usuario>, AppError> {
    storage::get_current_session(&state, window.label()).await
}

#[tauri::command]
pub async fn logout_usuario(
    window: Window,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    storage::logout_usuario(&state, window.label()).await
}

#[tauri::command]
pub async fn get_all_usuarios(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Vec<Usuario>, AppError> {
    storage::get_all_usuarios(&state, window.label()).await
}

#[tauri::command]
pub async fn actualizar_usuario(
    window: Window,
    state: State<'_, AppState>,
    id_usuario: String,
    request: UpdateUsuarioRequest,
) -> Result<Usuario, AppError> {
    storage::actualizar_usuario(&state, window.label(), &id_usuario, request).await
}

#[tauri::command]
pub async fn desactivar_usuario(
    window: Window,
    state: State<'_, AppState>,
    id_usuario: String,
) -> Result<Usuario, AppError> {
    storage::desactivar_usuario(&state, window.label(), &id_usuario).await
}

#[tauri::command]
pub async fn reactivar_usuario(
    window: Window,
    state: State<'_, AppState>,
    id_usuario: String,
) -> Result<Usuario, AppError> {
    storage::reactivar_usuario(&state, window.label(), &id_usuario).await
}