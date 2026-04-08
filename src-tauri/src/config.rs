use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseBackend {
    Sqlite,
    MongoDb,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub backend: DatabaseBackend,
    pub sqlite_url: String,
    pub mongodb_uri: Option<String>,
    pub mongodb_db_name: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        let sqlite_url = env::var("PJUPI_SQLITE_URL").unwrap_or_else(|_| "sqlite:database.db".to_string());
        let mongodb_uri = env::var("PJUPI_MONGODB_URI").ok();
        let backend_value = env::var("PJUPI_DB_BACKEND").ok();

        let backend = match backend_value.as_deref().map(|value| value.to_ascii_lowercase()) {
            Some(value) if value == "mongodb" || value == "mongo" => DatabaseBackend::MongoDb,
            Some(value) if value == "sqlite" => DatabaseBackend::Sqlite,
            _ if mongodb_uri.is_some() => DatabaseBackend::MongoDb,
            _ => DatabaseBackend::Sqlite,
        };

        let mongodb_db_name = env::var("PJUPI_MONGODB_DB").unwrap_or_else(|_| "pjupi".to_string());

        Self {
            backend,
            sqlite_url,
            mongodb_uri,
            mongodb_db_name,
        }
    }

    pub fn requires_mongodb(&self) -> bool {
        self.backend == DatabaseBackend::MongoDb
    }
}