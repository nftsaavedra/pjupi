use crate::config::DatabaseBackend;
use crate::error::AppError;
use crate::state::AppState;
use mongodb::Database;
use sqlx::SqlitePool;

/// Backend strategy abstraction to eliminate repeated `match state.backend` patterns
/// 
/// This trait defines a contract for backend operations, allowing services to be
/// backend-agnostic. The concrete implementation is selected at runtime based on
/// configuration.
/// 
/// Usage pattern in services:
/// ```
/// let strategy = get_backend_strategy(state)?;
/// let result = strategy.execute_operation(...).await?;
/// ```
/// 
/// This replaces:
/// ```
/// match state.primary_backend {
///     Sqlite => sqlite_operation(...),
///     MongoDb => mongo_operation(...),
/// }
/// ```
pub struct BackendStrategy {
    pub backend_type: DatabaseBackend,
    sqlite_pool: Option<SqlitePool>,
    mongo_db: Option<Database>,
}

impl BackendStrategy {
    pub fn new(
        backend_type: DatabaseBackend,
        sqlite_pool: Option<SqlitePool>,
        mongo_db: Option<Database>,
    ) -> Result<Self, AppError> {
        match backend_type {
            DatabaseBackend::Sqlite => {
                if sqlite_pool.is_none() {
                    return Err(AppError::InternalError(
                        "SQLite backend selected but no pool available".to_string(),
                    ));
                }
            }
            DatabaseBackend::MongoDb => {
                if mongo_db.is_none() {
                    return Err(AppError::InternalError(
                        "MongoDB backend selected but no connection available".to_string(),
                    ));
                }
            }
        }

        Ok(BackendStrategy {
            backend_type,
            sqlite_pool,
            mongo_db,
        })
    }

    /// Get the SQLite pool if primary backend is SQLite
    pub fn sqlite(&self) -> Result<&SqlitePool, AppError> {
        self.sqlite_pool
            .as_ref()
            .ok_or_else(|| AppError::InternalError("SQLite pool not available".to_string()))
    }

    /// Get the MongoDB database if primary backend is MongoDB
    pub fn mongo(&self) -> Result<&Database, AppError> {
        self.mongo_db
            .as_ref()
            .ok_or_else(|| AppError::InternalError("MongoDB connection not available".to_string()))
    }

    /// Check if the primary backend is MongoDB
    #[allow(dead_code)]
    pub fn is_mongo_primary(&self) -> bool {
        matches!(self.backend_type, DatabaseBackend::MongoDb)
    }

    /// Check if the primary backend is SQLite
    pub fn is_sqlite_primary(&self) -> bool {
        matches!(self.backend_type, DatabaseBackend::Sqlite)
    }
}

/// Factory function to create the appropriate backend strategy from AppState
pub fn get_backend_strategy(state: &AppState) -> Result<BackendStrategy, AppError> {
    BackendStrategy::new(
        state.primary_backend,
        state.sqlite.clone(),
        state.mongo.clone(),
    )
}
