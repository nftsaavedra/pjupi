use tauri::State;

use crate::domain::usuario::{AuthStatus, BootstrapUsuarioRequest, CreateUsuarioRequest, LoginUsuarioRequest, UpdateUsuarioRequest, Usuario};
use crate::error::AppError;
use crate::state::AppState;
use crate::storage;

#[tauri::command]
pub async fn crear_usuario(
    state: State<'_, AppState>,
    actor_user_id: String,
    request: CreateUsuarioRequest,
) -> Result<Usuario, AppError> {
    storage::crear_usuario(&state, &actor_user_id, request).await
}

#[tauri::command]
pub async fn get_auth_status(
    state: State<'_, AppState>,
) -> Result<AuthStatus, AppError> {
    storage::get_auth_status(&state).await
}

#[tauri::command]
pub async fn registrar_primer_usuario(
    state: State<'_, AppState>,
    request: BootstrapUsuarioRequest,
) -> Result<Usuario, AppError> {
    storage::registrar_primer_usuario(&state, request).await
}

#[tauri::command]
pub async fn login_usuario(
    state: State<'_, AppState>,
    request: LoginUsuarioRequest,
) -> Result<Usuario, AppError> {
    storage::login_usuario(&state, request).await
}

#[tauri::command]
pub async fn get_all_usuarios(
    state: State<'_, AppState>,
    actor_user_id: String,
) -> Result<Vec<Usuario>, AppError> {
    storage::get_all_usuarios(&state, &actor_user_id).await
}

#[tauri::command]
pub async fn actualizar_usuario(
    state: State<'_, AppState>,
    actor_user_id: String,
    id_usuario: String,
    request: UpdateUsuarioRequest,
) -> Result<Usuario, AppError> {
    storage::actualizar_usuario(&state, &actor_user_id, &id_usuario, request).await
}

#[tauri::command]
pub async fn desactivar_usuario(
    state: State<'_, AppState>,
    actor_user_id: String,
    id_usuario: String,
) -> Result<Usuario, AppError> {
    storage::desactivar_usuario(&state, &actor_user_id, &id_usuario).await
}

#[tauri::command]
pub async fn reactivar_usuario(
    state: State<'_, AppState>,
    actor_user_id: String,
    id_usuario: String,
) -> Result<Usuario, AppError> {
    storage::reactivar_usuario(&state, &actor_user_id, &id_usuario).await
}