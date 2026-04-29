use crate::domain::usuario::{
    AuthStatus,
    BootstrapUsuarioRequest,
    CreateUsuarioRequest,
    LoginUsuarioRequest,
    UpdateUsuarioRequest,
    Usuario,
};
use crate::error::AppError;
use crate::infrastructure::mongo_repo;
use crate::state::AppState;

pub async fn create(state: &AppState, actor_user_id: &str, request: CreateUsuarioRequest) -> Result<Usuario, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::create_usuario(db, actor_user_id, request).await
}

pub async fn get_auth_status(state: &AppState) -> Result<AuthStatus, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_auth_status(db).await
}

pub async fn bootstrap_admin(state: &AppState, request: BootstrapUsuarioRequest) -> Result<Usuario, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::bootstrap_admin(db, request).await
}

pub async fn login(state: &AppState, request: LoginUsuarioRequest) -> Result<Usuario, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::login_usuario(db, request).await
}

pub async fn get_all(state: &AppState, actor_user_id: &str) -> Result<Vec<Usuario>, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_all_usuarios(db, actor_user_id).await
}

pub async fn get_by_id_public(state: &AppState, user_id: &str) -> Result<Usuario, AppError> {
    let db = state.mongo_db()?;
    Ok(mongo_repo::get_usuario_by_id(db, user_id).await?.public_view())
}

pub async fn update(
    state: &AppState,
    actor_user_id: &str,
    id_usuario: &str,
    request: UpdateUsuarioRequest,
) -> Result<Usuario, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::update_usuario(db, actor_user_id, id_usuario, request).await
}

pub async fn deactivate(state: &AppState, actor_user_id: &str, id_usuario: &str) -> Result<Usuario, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::desactivar_usuario(db, actor_user_id, id_usuario).await
}

pub async fn reactivate(state: &AppState, actor_user_id: &str, id_usuario: &str) -> Result<Usuario, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::reactivar_usuario(db, actor_user_id, id_usuario).await
}
