use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use sqlx::{query, query_as, SqlitePool};

use crate::domain::usuario::{AuthStatus, BootstrapUsuarioRequest, CreateUsuarioRequest, LoginUsuarioRequest, UpdateUsuarioRequest, Usuario, UsuarioConPassword};
use crate::error::AppError;

fn validar_usuario(username: &str, nombre_completo: &str, rol: &str) -> Result<(), AppError> {
    if username.trim().is_empty() || nombre_completo.trim().is_empty() || rol.trim().is_empty() {
        return Err(AppError::InternalError("Complete todos los campos del usuario.".to_string()));
    }

    if !matches!(rol.trim(), "admin" | "operador" | "consulta") {
        return Err(AppError::InternalError("El rol del usuario no es válido.".to_string()));
    }

    Ok(())
}

fn hash_password(password: &str) -> Result<String, AppError> {
    if password.trim().len() < 8 {
        return Err(AppError::InternalError("La contraseña debe tener al menos 8 caracteres.".to_string()));
    }

    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|err| AppError::InternalError(format!("No se pudo proteger la contraseña: {err}")))
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|err| AppError::InternalError(format!("No se pudo leer la contraseña protegida: {err}")))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

async fn count_usuarios(pool: &SqlitePool) -> Result<i64, AppError> {
    let total: (i64,) = query_as("SELECT COUNT(*) FROM usuario")
        .fetch_one(pool)
        .await?;
    Ok(total.0)
}

async fn get_usuario_by_username(pool: &SqlitePool, username: &str) -> Result<UsuarioConPassword, AppError> {
    let usuario = query_as::<_, UsuarioConPassword>(
        "SELECT id_usuario, username, nombre_completo, rol, password_hash, activo FROM usuario WHERE username = ?"
    )
    .bind(username.trim().to_lowercase())
    .fetch_one(pool)
    .await?;
    Ok(usuario)
}

pub async fn get_auth_status(pool: &SqlitePool) -> Result<AuthStatus, AppError> {
    let total = count_usuarios(pool).await?;
    Ok(AuthStatus {
        has_users: total > 0,
        requires_setup: total == 0,
    })
}

pub async fn create_usuario(pool: &SqlitePool, request: CreateUsuarioRequest) -> Result<Usuario, AppError> {
    validar_usuario(&request.username, &request.nombre_completo, &request.rol)?;
    let password_hash = hash_password(&request.password)?;
    let usuario = UsuarioConPassword::new(request, password_hash);

    query(
        "INSERT INTO usuario (id_usuario, username, nombre_completo, rol, password_hash, activo) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&usuario.id_usuario)
    .bind(&usuario.username)
    .bind(&usuario.nombre_completo)
    .bind(&usuario.rol)
    .bind(&usuario.password_hash)
    .bind(usuario.activo)
    .execute(pool)
    .await?;

    Ok(usuario.public_view())
}

pub async fn bootstrap_admin(pool: &SqlitePool, request: BootstrapUsuarioRequest) -> Result<Usuario, AppError> {
    if count_usuarios(pool).await? > 0 {
        return Err(AppError::InternalError("La configuración inicial ya fue completada.".to_string()));
    }

    create_usuario(
        pool,
        CreateUsuarioRequest {
            username: request.username,
            nombre_completo: request.nombre_completo,
            rol: "admin".to_string(),
            password: request.password,
        },
    )
    .await
}

pub async fn login_usuario(pool: &SqlitePool, request: LoginUsuarioRequest) -> Result<Usuario, AppError> {
    let usuario = get_usuario_by_username(pool, &request.username).await?;

    if usuario.activo == 0 {
        return Err(AppError::InternalError("El usuario está inactivo.".to_string()));
    }

    if !verify_password(&request.password, &usuario.password_hash)? {
        return Err(AppError::InternalError("Usuario o contraseña incorrectos.".to_string()));
    }

    Ok(usuario.public_view())
}

pub async fn get_all_usuarios(pool: &SqlitePool) -> Result<Vec<Usuario>, AppError> {
    let usuarios = query_as::<_, Usuario>(
        "SELECT id_usuario, username, nombre_completo, rol, activo FROM usuario ORDER BY username"
    )
    .fetch_all(pool)
    .await?;
    Ok(usuarios)
}

pub async fn get_usuario_by_id(pool: &SqlitePool, id_usuario: &str) -> Result<UsuarioConPassword, AppError> {
    let usuario = query_as::<_, UsuarioConPassword>(
        "SELECT id_usuario, username, nombre_completo, rol, password_hash, activo FROM usuario WHERE id_usuario = ?"
    )
    .bind(id_usuario)
    .fetch_one(pool)
    .await?;
    Ok(usuario)
}

pub async fn update_usuario(pool: &SqlitePool, id_usuario: &str, request: UpdateUsuarioRequest) -> Result<Usuario, AppError> {
    validar_usuario(&request.username, &request.nombre_completo, &request.rol)?;

    match request.password.as_deref().filter(|value| !value.trim().is_empty()) {
        Some(password) => {
            let password_hash = hash_password(password)?;
            query(
                "UPDATE usuario SET username = ?, nombre_completo = ?, rol = ?, password_hash = ? WHERE id_usuario = ?"
            )
            .bind(request.username.trim().to_lowercase())
            .bind(request.nombre_completo.trim())
            .bind(request.rol.trim())
            .bind(password_hash)
            .bind(id_usuario)
            .execute(pool)
            .await?;
        }
        None => {
            query(
                "UPDATE usuario SET username = ?, nombre_completo = ?, rol = ? WHERE id_usuario = ?"
            )
            .bind(request.username.trim().to_lowercase())
            .bind(request.nombre_completo.trim())
            .bind(request.rol.trim())
            .bind(id_usuario)
            .execute(pool)
            .await?;
        }
    }

    Ok(get_usuario_by_id(pool, id_usuario).await?.public_view())
}

pub async fn desactivar_usuario(pool: &SqlitePool, id_usuario: &str) -> Result<Usuario, AppError> {
    query("UPDATE usuario SET activo = 0 WHERE id_usuario = ?")
        .bind(id_usuario)
        .execute(pool)
        .await?;

    Ok(get_usuario_by_id(pool, id_usuario).await?.public_view())
}

pub async fn reactivar_usuario(pool: &SqlitePool, id_usuario: &str) -> Result<Usuario, AppError> {
    query("UPDATE usuario SET activo = 1 WHERE id_usuario = ?")
        .bind(id_usuario)
        .execute(pool)
        .await?;

    Ok(get_usuario_by_id(pool, id_usuario).await?.public_view())
}