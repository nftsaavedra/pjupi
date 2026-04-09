use mongodb::Database;
use sqlx::SqlitePool;

use crate::config::{DatabaseBackend, ReniecConfig};
use crate::error::AppError;

pub struct AppState {
    pub backend: DatabaseBackend,
    pub sqlite: Option<SqlitePool>,
    pub mongo: Option<Database>,
    pub reniec: ReniecConfig,
}

impl AppState {
    pub fn new(backend: DatabaseBackend, sqlite: Option<SqlitePool>, mongo: Option<Database>, reniec: ReniecConfig) -> Self {
        Self { backend, sqlite, mongo, reniec }
    }

    pub fn sqlite_pool(&self) -> Result<&SqlitePool, AppError> {
        self.sqlite.as_ref().ok_or_else(|| {
            AppError::ConfigurationError("SQLite no está inicializado para la configuración actual.".to_string())
        })
    }

    pub fn mongo_db(&self) -> Result<&Database, AppError> {
        self.mongo.as_ref().ok_or_else(|| {
            AppError::ConfigurationError("MongoDB no está inicializado para la configuración actual.".to_string())
        })
    }

    pub fn reniec_config(&self) -> &ReniecConfig {
        &self.reniec
    }
}