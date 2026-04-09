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

#[derive(Debug, Clone)]
pub struct ReniecConfig {
    pub api_base_url: String,
    pub token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RenacytConfig {
    pub api_base_url: String,
    pub acto_version: String,
    pub ficha_base_url: String,
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

impl ReniecConfig {
    pub fn from_env() -> Self {
        let api_base_url = env::var("PJUPI_RENIEC_API_BASE_URL")
            .unwrap_or_else(|_| "https://api.decolecta.com/v1".to_string());
        let token = env::var("PJUPI_RENIEC_TOKEN")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        Self { api_base_url, token }
    }
}

impl RenacytConfig {
    pub fn from_env() -> Self {
        let api_base_url = env::var("PJUPI_RENACYT_API_BASE_URL")
            .unwrap_or_else(|_| "https://renacyt.concytec.gob.pe/renacyt-backend".to_string());
        let acto_version = env::var("PJUPI_RENACYT_ACTO_VERSION")
            .unwrap_or_else(|_| "2021".to_string());
        let ficha_base_url = env::var("PJUPI_RENACYT_FICHA_BASE_URL")
            .unwrap_or_else(|_| "https://servicio-renacyt.concytec.gob.pe/ficha-renacyt/".to_string());

        Self {
            api_base_url,
            acto_version,
            ficha_base_url,
        }
    }
}