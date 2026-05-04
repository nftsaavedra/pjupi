use futures_util::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::Database;
use rand_core::OsRng;

use crate::shared::error::AppError;
use crate::usuarios::models::{
    AuthStatus,
    BootstrapUsuarioRequest,
    CreateUsuarioRequest,
    LoginUsuarioRequest,
    UpdateUsuarioRequest,
    Usuario,
    UsuarioConPassword,
};

async fn load_usuarios(db: &Database) -> Result<Vec<UsuarioConPassword>, AppError> {
    db.collection::<UsuarioConPassword>("usuarios")
        .find(doc! {})
        .await?
        .try_collect::<Vec<_>>()
        .await
        .map_err(Into::into)
}

async fn count_usuarios(db: &Database) -> Result<u64, AppError> {
    db.collection::<Document>("usuarios")
        .count_documents(doc! {})
        .await
        .map_err(Into::into)
}

async fn get_usuario_by_username(db: &Database, username: &str) -> Result<UsuarioConPassword, AppError> {
    db.collection::<UsuarioConPassword>("usuarios")
        .find_one(doc! { "username": username.trim().to_lowercase() })
        .await?
        .ok_or_else(|| AppError::NotFound("Usuario no encontrado.".to_string()))
}

fn validar_usuario(username: &str, nombre_completo: &str, rol: &str) -> Result<(), AppError> {
    if username.trim().is_empty() || nombre_completo.trim().is_empty() || rol.trim().is_empty() {
        return Err(AppError::InternalError("Complete todos los campos del usuario.".to_string()));
    }
    if !matches!(rol.trim(), "admin" | "operador" | "consulta") {
        return Err(AppError::InternalError("El rol del usuario no es válido.".to_string()));
    }
    Ok(())
}

async fn validar_actor_admin(db: &Database, actor_user_id: &str) -> Result<UsuarioConPassword, AppError> {
    let actor = db.collection::<UsuarioConPassword>("usuarios")
        .find_one(doc! { "id_usuario": actor_user_id })
        .await?
        .ok_or_else(|| AppError::NotFound("Usuario no encontrado.".to_string()))?;

    if actor.activo == 0 {
        return Err(AppError::InternalError("El usuario actual está inactivo y no puede administrar accesos.".to_string()));
    }

    if actor.rol.trim() != "admin" {
        return Err(AppError::InternalError("No tiene permisos para administrar usuarios.".to_string()));
    }

    Ok(actor)
}

fn hash_password(password: &str) -> Result<String, AppError> {
    use argon2::{password_hash::{PasswordHasher, SaltString}, Argon2};

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
    use argon2::{password_hash::PasswordHash, password_hash::PasswordVerifier, Argon2};

    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|err| AppError::InternalError(format!("No se pudo leer la contraseña protegida: {err}")))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub async fn get_auth_status(db: &Database) -> Result<AuthStatus, AppError> {
    let total = count_usuarios(db).await?;
    Ok(AuthStatus {
        has_users: total > 0,
        requires_setup: total == 0,
    })
}

pub async fn create_usuario(db: &Database, actor_user_id: &str, request: CreateUsuarioRequest) -> Result<Usuario, AppError> {
    validar_actor_admin(db, actor_user_id).await?;
    validar_usuario(&request.username, &request.nombre_completo, &request.rol)?;
    let password_hash = hash_password(&request.password)?;
    let usuario = UsuarioConPassword::new(request, password_hash);
    db.collection::<UsuarioConPassword>("usuarios").insert_one(&usuario).await?;
    Ok(usuario.public_view())
}

pub async fn bootstrap_admin(db: &Database, request: BootstrapUsuarioRequest) -> Result<Usuario, AppError> {
    if count_usuarios(db).await? > 0 {
        return Err(AppError::InternalError("La configuración inicial ya fue completada.".to_string()));
    }

    validar_usuario(&request.username, &request.nombre_completo, "admin")?;
    let password_hash = hash_password(&request.password)?;
    let usuario = UsuarioConPassword::new(
        CreateUsuarioRequest {
            username: request.username,
            nombre_completo: request.nombre_completo,
            rol: "admin".to_string(),
            password: request.password,
        },
        password_hash,
    );

    db.collection::<UsuarioConPassword>("usuarios").insert_one(&usuario).await?;
    Ok(usuario.public_view())
}

pub async fn login_usuario(db: &Database, request: LoginUsuarioRequest) -> Result<Usuario, AppError> {
    let usuario = get_usuario_by_username(db, &request.username).await?;

    if usuario.activo == 0 {
        return Err(AppError::InternalError("El usuario está inactivo.".to_string()));
    }

    if !verify_password(&request.password, &usuario.password_hash)? {
        return Err(AppError::InternalError("Usuario o contraseña incorrectos.".to_string()));
    }

    Ok(usuario.public_view())
}

pub async fn get_all_usuarios(db: &Database, actor_user_id: &str) -> Result<Vec<Usuario>, AppError> {
    validar_actor_admin(db, actor_user_id).await?;
    let mut usuarios: Vec<Usuario> = load_usuarios(db)
        .await?
        .into_iter()
        .map(|usuario| usuario.public_view())
        .collect();
    usuarios.sort_by(|a, b| a.username.cmp(&b.username));
    Ok(usuarios)
}

pub async fn get_usuario_by_id(db: &Database, id_usuario: &str) -> Result<UsuarioConPassword, AppError> {
    db.collection::<UsuarioConPassword>("usuarios")
        .find_one(doc! { "id_usuario": id_usuario })
        .await?
        .ok_or_else(|| AppError::NotFound("Usuario no encontrado.".to_string()))
}

pub async fn update_usuario(db: &Database, actor_user_id: &str, id_usuario: &str, request: UpdateUsuarioRequest) -> Result<Usuario, AppError> {
    validar_actor_admin(db, actor_user_id).await?;
    validar_usuario(&request.username, &request.nombre_completo, &request.rol)?;

    if actor_user_id == id_usuario {
        let usuario_actual = db.collection::<UsuarioConPassword>("usuarios")
            .find_one(doc! { "id_usuario": id_usuario })
            .await?
            .ok_or_else(|| AppError::NotFound("Usuario no encontrado.".to_string()))?;

        if usuario_actual.rol.trim() != request.rol.trim() {
            return Err(AppError::InternalError("No puede cambiar su propio rol. Solicite a otro administrador que lo haga.".to_string()));
        }
    }

    let mut updates = doc! {
        "username": request.username.trim().to_lowercase(),
        "nombre_completo": request.nombre_completo.trim(),
        "rol": request.rol.trim(),
    };

    if let Some(password) = request.password.as_deref().filter(|value| !value.trim().is_empty()) {
        updates.insert("password_hash", hash_password(password)?);
    }

    db.collection::<Document>("usuarios")
        .update_one(doc! { "id_usuario": id_usuario }, doc! { "$set": updates })
        .await?;

    let usuario = db.collection::<UsuarioConPassword>("usuarios")
        .find_one(doc! { "id_usuario": id_usuario })
        .await?
        .ok_or_else(|| AppError::NotFound("Usuario no encontrado.".to_string()))?;
    Ok(usuario.public_view())
}

pub async fn desactivar_usuario(db: &Database, actor_user_id: &str, id_usuario: &str) -> Result<Usuario, AppError> {
    validar_actor_admin(db, actor_user_id).await?;

    if actor_user_id == id_usuario {
        return Err(AppError::InternalError("No puede cambiar el estado de su propio usuario.".to_string()));
    }

    db.collection::<Document>("usuarios")
        .update_one(doc! { "id_usuario": id_usuario }, doc! { "$set": { "activo": 0i64 } })
        .await?;

    let usuario = db.collection::<UsuarioConPassword>("usuarios")
        .find_one(doc! { "id_usuario": id_usuario })
        .await?
        .ok_or_else(|| AppError::NotFound("Usuario no encontrado.".to_string()))?;
    Ok(usuario.public_view())
}

pub async fn reactivar_usuario(db: &Database, actor_user_id: &str, id_usuario: &str) -> Result<Usuario, AppError> {
    validar_actor_admin(db, actor_user_id).await?;

    if actor_user_id == id_usuario {
        return Err(AppError::InternalError("No puede cambiar el estado de su propio usuario.".to_string()));
    }

    db.collection::<Document>("usuarios")
        .update_one(doc! { "id_usuario": id_usuario }, doc! { "$set": { "activo": 1i64 } })
        .await?;

    let usuario = db.collection::<UsuarioConPassword>("usuarios")
        .find_one(doc! { "id_usuario": id_usuario })
        .await?
        .ok_or_else(|| AppError::NotFound("Usuario no encontrado.".to_string()))?;
    Ok(usuario.public_view())
}
