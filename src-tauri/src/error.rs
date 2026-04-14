use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum AppError {
    DatabaseError(String),
    UniqueConstraintViolation(String),
    NotFound(String),
    InternalError(String),
    ConfigurationError(String),
    ExternalServiceError(String),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::Database(db_err) => {
                if db_err.code() == Some("2067".into()) { // UNIQUE constraint failed
                    let message = db_err.message().to_string();
                    let user_message = if message.contains("docente.dni") {
                        "El DNI ingresado ya existe en el padrón.".to_string()
                    } else if message.contains("grado_academico.nombre") {
                        "Ya existe un grado académico con ese nombre.".to_string()
                    } else if message.contains("usuario.username") {
                        "El nombre de usuario ya existe.".to_string()
                    } else {
                        "Ya existe un registro con un valor único duplicado.".to_string()
                    };
                    AppError::UniqueConstraintViolation(user_message)
                } else {
                    AppError::DatabaseError(db_err.message().to_string())
                }
            }
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(err: mongodb::error::Error) -> Self {
        let message = err.to_string();
        let lowered = message.to_lowercase();
        if message.contains("E11000") || lowered.contains("duplicate key") {
            let user_message = if lowered.contains("username") {
                "El nombre de usuario ya existe.".to_string()
            } else if lowered.contains("dni") {
                "El DNI ingresado ya existe en el padrón.".to_string()
            } else if lowered.contains("nombre") {
                "Ya existe un registro con ese nombre.".to_string()
            } else {
                "Ya existe un registro con un valor único duplicado.".to_string()
            };
            AppError::UniqueConstraintViolation(user_message)
        } else {
            AppError::DatabaseError(message)
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::ExternalServiceError(err.to_string())
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(message)
            | AppError::UniqueConstraintViolation(message)
            | AppError::NotFound(message)
            | AppError::InternalError(message)
            | AppError::ConfigurationError(message)
            | AppError::ExternalServiceError(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for AppError {}