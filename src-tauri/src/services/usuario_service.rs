use crate::domain::usuario::{
    AuthStatus,
    BootstrapUsuarioRequest,
    CreateUsuarioRequest,
    LoginUsuarioRequest,
    UpdateUsuarioRequest,
    Usuario,
};
use crate::error::AppError;
use crate::infrastructure::{mongo_repo, usuario_repo, sync_outbox_repo};
use crate::services::backend_strategy::get_backend_strategy;
use crate::state::AppState;

async fn enqueue_usuario_snapshot(state: &AppState, usuario: &Usuario) -> Result<(), AppError> {
    // Serialize the public Usuario (without password_hash) as snapshot
    let payload_json = serde_json::to_string(usuario)
        .map_err(|error| AppError::InternalError(format!("No se pudo serializar snapshot offline de usuario: {}", error)))?;

    sync_outbox_repo::enqueue_operation(
        state.sqlite_pool()?,
        "usuario",
        &usuario.id_usuario,
        "usuario.snapshot",
        &payload_json,
    )
    .await
}

pub async fn create(state: &AppState, actor_user_id: &str, request: CreateUsuarioRequest) -> Result<Usuario, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        let usuario = usuario_repo::create_usuario(strategy.sqlite()?, actor_user_id, request).await?;
        enqueue_usuario_snapshot(state, &usuario).await?;
        Ok(usuario)
    } else {
        mongo_repo::create_usuario(strategy.mongo()?, actor_user_id, request).await
    }
}

pub async fn get_auth_status(state: &AppState) -> Result<AuthStatus, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        usuario_repo::get_auth_status(strategy.sqlite()?).await
    } else {
        mongo_repo::get_auth_status(strategy.mongo()?).await
    }
}

pub async fn bootstrap_admin(state: &AppState, request: BootstrapUsuarioRequest) -> Result<Usuario, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        let usuario = usuario_repo::bootstrap_admin(strategy.sqlite()?, request).await?;
        enqueue_usuario_snapshot(state, &usuario).await?;
        Ok(usuario)
    } else {
        mongo_repo::bootstrap_admin(strategy.mongo()?, request).await
    }
}

pub async fn login(state: &AppState, request: LoginUsuarioRequest) -> Result<Usuario, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        usuario_repo::login_usuario(strategy.sqlite()?, request).await
    } else {
        mongo_repo::login_usuario(strategy.mongo()?, request).await
    }
}

pub async fn get_all(state: &AppState, actor_user_id: &str) -> Result<Vec<Usuario>, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        usuario_repo::get_all_usuarios(strategy.sqlite()?, actor_user_id).await
    } else {
        mongo_repo::get_all_usuarios(strategy.mongo()?, actor_user_id).await
    }
}

pub async fn get_by_id_public(state: &AppState, user_id: &str) -> Result<Usuario, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        Ok(usuario_repo::get_usuario_by_id(strategy.sqlite()?, user_id).await?.public_view())
    } else {
        Ok(mongo_repo::get_usuario_by_id(strategy.mongo()?, user_id).await?.public_view())
    }
}

pub async fn update(
    state: &AppState,
    actor_user_id: &str,
    id_usuario: &str,
    request: UpdateUsuarioRequest,
) -> Result<Usuario, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        let usuario = usuario_repo::update_usuario(strategy.sqlite()?, actor_user_id, id_usuario, request).await?;
        enqueue_usuario_snapshot(state, &usuario).await?;
        Ok(usuario)
    } else {
        mongo_repo::update_usuario(strategy.mongo()?, actor_user_id, id_usuario, request).await
    }
}

pub async fn deactivate(state: &AppState, actor_user_id: &str, id_usuario: &str) -> Result<Usuario, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        let usuario = usuario_repo::desactivar_usuario(strategy.sqlite()?, actor_user_id, id_usuario).await?;
        enqueue_usuario_snapshot(state, &usuario).await?;
        Ok(usuario)
    } else {
        mongo_repo::desactivar_usuario(strategy.mongo()?, actor_user_id, id_usuario).await
    }
}

pub async fn reactivate(state: &AppState, actor_user_id: &str, id_usuario: &str) -> Result<Usuario, AppError> {
    let strategy = get_backend_strategy(state)?;

    if strategy.is_sqlite_primary() {
        let usuario = usuario_repo::reactivar_usuario(strategy.sqlite()?, actor_user_id, id_usuario).await?;
        enqueue_usuario_snapshot(state, &usuario).await?;
        Ok(usuario)
    } else {
        mongo_repo::reactivar_usuario(strategy.mongo()?, actor_user_id, id_usuario).await
    }
}