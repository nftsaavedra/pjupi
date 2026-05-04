use tauri::{State, Window};

use crate::shared::error::AppError;
use crate::shared::state::AppState;
use crate::shared::access_control;
use crate::usuarios::models::{AuthStatus, BootstrapUsuarioRequest, CreateUsuarioRequest, LoginUsuarioRequest, UpdateUsuarioRequest, Usuario};

#[tauri::command]
pub async fn crear_usuario(
    window: Window,
    state: State<'_, AppState>,
    request: CreateUsuarioRequest,
) -> Result<Usuario, AppError> {
    access_control::crear_usuario(&state, window.label(), request).await
}

#[tauri::command]
pub async fn get_auth_status(
    state: State<'_, AppState>,
) -> Result<AuthStatus, AppError> {
    access_control::get_auth_status(&state).await
}

#[tauri::command]
pub async fn registrar_primer_usuario(
    window: Window,
    state: State<'_, AppState>,
    request: BootstrapUsuarioRequest,
) -> Result<Usuario, AppError> {
    access_control::registrar_primer_usuario(&state, window.label(), request).await
}

#[tauri::command]
pub async fn login_usuario(
    window: Window,
    state: State<'_, AppState>,
    request: LoginUsuarioRequest,
) -> Result<Usuario, AppError> {
    access_control::login_usuario(&state, window.label(), request).await
}

#[tauri::command]
pub async fn get_current_session(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Option<Usuario>, AppError> {
    access_control::get_current_session(&state, window.label()).await
}

#[tauri::command]
pub async fn logout_usuario(
    window: Window,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    access_control::logout_usuario(&state, window.label()).await
}

#[tauri::command]
pub async fn get_all_usuarios(
    window: Window,
    state: State<'_, AppState>,
) -> Result<Vec<Usuario>, AppError> {
    access_control::get_all_usuarios(&state, window.label()).await
}

#[tauri::command]
pub async fn actualizar_usuario(
    window: Window,
    state: State<'_, AppState>,
    id_usuario: String,
    request: UpdateUsuarioRequest,
) -> Result<Usuario, AppError> {
    access_control::actualizar_usuario(&state, window.label(), &id_usuario, request).await
}

#[tauri::command]
pub async fn desactivar_usuario(
    window: Window,
    state: State<'_, AppState>,
    id_usuario: String,
) -> Result<Usuario, AppError> {
    access_control::desactivar_usuario(&state, window.label(), &id_usuario).await
}

#[tauri::command]
pub async fn reactivar_usuario(
    window: Window,
    state: State<'_, AppState>,
    id_usuario: String,
) -> Result<Usuario, AppError> {
    access_control::reactivar_usuario(&state, window.label(), &id_usuario).await
}
