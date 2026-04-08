use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Usuario {
    pub id_usuario: String,
    pub username: String,
    pub nombre_completo: String,
    pub rol: String,
    pub activo: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct UsuarioConPassword {
    pub id_usuario: String,
    pub username: String,
    pub nombre_completo: String,
    pub rol: String,
    pub password_hash: String,
    pub activo: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateUsuarioRequest {
    pub username: String,
    pub nombre_completo: String,
    pub rol: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct BootstrapUsuarioRequest {
    pub username: String,
    pub nombre_completo: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUsuarioRequest {
    pub username: String,
    pub nombre_completo: String,
    pub rol: String,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginUsuarioRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthStatus {
    pub has_users: bool,
    pub requires_setup: bool,
}

impl UsuarioConPassword {
    pub fn new(request: CreateUsuarioRequest, password_hash: String) -> Self {
        Self {
            id_usuario: Uuid::new_v4().to_string(),
            username: request.username.trim().to_lowercase(),
            nombre_completo: request.nombre_completo.trim().to_string(),
            rol: request.rol.trim().to_string(),
            password_hash,
            activo: 1,
        }
    }

    pub fn public_view(&self) -> Usuario {
        Usuario {
            id_usuario: self.id_usuario.clone(),
            username: self.username.clone(),
            nombre_completo: self.nombre_completo.clone(),
            rol: self.rol.clone(),
            activo: self.activo,
        }
    }
}