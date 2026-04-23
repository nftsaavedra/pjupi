use std::{
    collections::HashMap,
    env,
    fs,
    path::{Path, PathBuf},
};

use crate::error::AppError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseBackend {
    Sqlite,
    MongoDb,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub primary_backend: DatabaseBackend,
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

#[derive(Debug, Clone)]
pub struct PureConfig {
    pub api_base_url: String,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub database: DatabaseConfig,
    pub reniec: ReniecConfig,
    pub renacyt: RenacytConfig,
    pub pure: PureConfig,
    #[allow(dead_code)]
    pub user_env_path: PathBuf,
}

impl DatabaseConfig {
    pub fn from_values(values: &HashMap<String, String>) -> Self {
        let sqlite_url = normalize_sqlite_url(values.get("PJUPI_SQLITE_URL").map(String::as_str));
        let mongodb_uri = env::var("PJUPI_MONGODB_URI").ok();
        let mongodb_uri = values
            .get("PJUPI_MONGODB_URI")
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .or(mongodb_uri);
        let backend_value = values.get("PJUPI_DB_BACKEND").cloned();

        let primary_backend = match backend_value.as_deref().map(|value| value.to_ascii_lowercase()) {
            Some(value) if value == "mongodb" || value == "mongo" => DatabaseBackend::MongoDb,
            Some(value) if value == "sqlite" => DatabaseBackend::Sqlite,
            _ if mongodb_uri.is_some() => DatabaseBackend::MongoDb,
            _ => DatabaseBackend::Sqlite,
        };

        let mongodb_db_name = values
            .get("PJUPI_MONGODB_DB")
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "pjupi".to_string());

        Self {
            primary_backend,
            sqlite_url,
            mongodb_uri,
            mongodb_db_name,
        }
    }

    pub fn requires_mongodb(&self) -> bool {
        self.primary_backend == DatabaseBackend::MongoDb
    }

    pub fn should_init_local_sqlite(&self) -> bool {
        true
    }
}

fn default_sqlite_url() -> String {
    let app_dir = resolve_local_data_dir().join("pjupi");

    if let Err(error) = fs::create_dir_all(&app_dir) {
        eprintln!("No se pudo crear el directorio local de datos en {:?}: {}", app_dir, error);
        return "sqlite:database.db".to_string();
    }

    let database_path = app_dir.join("database.db");
    sqlite_url_from_path(&database_path)
}

fn sqlite_url_from_path(path: &Path) -> String {
    let normalized = path.to_string_lossy().replace('\\', "/");

    if normalized.starts_with('/') {
        format!("sqlite://{}", normalized)
    } else {
        format!("sqlite:///{}", normalized)
    }
}

fn resolve_local_data_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .or_else(|| env::var_os("APPDATA").map(PathBuf::from))
            .unwrap_or_else(env::temp_dir)
    }

    #[cfg(target_os = "macos")]
    {
        env::var_os("HOME")
            .map(PathBuf::from)
            .map(|home| home.join("Library").join("Application Support"))
            .unwrap_or_else(env::temp_dir)
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| {
                env::var_os("HOME")
                    .map(PathBuf::from)
                    .map(|home| home.join(".local").join("share"))
            })
            .unwrap_or_else(env::temp_dir)
    }
}

impl ReniecConfig {
    pub fn from_values(values: &HashMap<String, String>) -> Self {
        let api_base_url = values
            .get("PJUPI_RENIEC_API_BASE_URL")
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "https://api.decolecta.com/v1".to_string());
        let token = values
            .get("PJUPI_RENIEC_TOKEN")
            .cloned()
            .or_else(|| env::var("PJUPI_RENIEC_TOKEN").ok())
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        Self { api_base_url, token }
    }
}

impl RenacytConfig {
    pub fn from_values(values: &HashMap<String, String>) -> Self {
        let api_base_url = values
            .get("PJUPI_RENACYT_API_BASE_URL")
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "https://renacyt.concytec.gob.pe/renacyt-backend".to_string());
        let acto_version = values
            .get("PJUPI_RENACYT_ACTO_VERSION")
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "2021".to_string());
        let ficha_base_url = values
            .get("PJUPI_RENACYT_FICHA_BASE_URL")
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "https://servicio-renacyt.concytec.gob.pe/ficha-renacyt/".to_string());

        Self {
            api_base_url,
            acto_version,
            ficha_base_url,
        }
    }
}

impl PureConfig {
    pub fn from_values(values: &HashMap<String, String>) -> Self {
        let api_base_url = values
            .get("PJUPI_PURE_API_BASE_URL")
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| "https://pure.unf.edu.pe/ws/api".to_string());
        let api_key = values
            .get("PJUPI_PURE_API_KEY")
            .cloned()
            .or_else(|| env::var("PJUPI_PURE_API_KEY").ok())
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty());

        Self { api_base_url, api_key }
    }
}

pub fn load_runtime_config(user_env_path: &Path, bundled_default_env_path: Option<&Path>) -> Result<RuntimeConfig, AppError> {
    if let Some(parent) = user_env_path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            AppError::ConfigurationError(format!("No se pudo preparar el directorio de configuración local: {}", error))
        })?;
    }

    if !user_env_path.exists() {
        if let Some(default_env_path) = bundled_default_env_path {
            fs::copy(default_env_path, user_env_path).map_err(|error| {
                AppError::ConfigurationError(format!("No se pudo copiar la configuración inicial de la aplicación: {}", error))
            })?;
        }
    }

    let mut values = HashMap::new();

    if let Some(default_env_path) = bundled_default_env_path {
        merge_env_file(&mut values, default_env_path)?;
    }

    if user_env_path.exists() {
        merge_env_file(&mut values, user_env_path)?;
    }

    merge_process_env(&mut values);

    Ok(RuntimeConfig {
        database: DatabaseConfig::from_values(&values),
        reniec: ReniecConfig::from_values(&values),
        renacyt: RenacytConfig::from_values(&values),
        pure: PureConfig::from_values(&values),
        user_env_path: user_env_path.to_path_buf(),
    })
}

fn normalize_sqlite_url(raw_value: Option<&str>) -> String {
    match raw_value.map(str::trim) {
        Some("") | None => default_sqlite_url(),
        Some("sqlite:database.db") | Some("sqlite://database.db") | Some("sqlite:///database.db") => default_sqlite_url(),
        Some(value) => value.to_string(),
    }
}

fn merge_process_env(values: &mut HashMap<String, String>) {
    for key in [
        "PJUPI_DB_BACKEND",
        "PJUPI_MONGODB_URI",
        "PJUPI_MONGODB_DB",
        "PJUPI_SQLITE_URL",
        "PJUPI_RENIEC_API_BASE_URL",
        "PJUPI_RENIEC_TOKEN",
        "PJUPI_RENACYT_API_BASE_URL",
        "PJUPI_RENACYT_ACTO_VERSION",
        "PJUPI_RENACYT_FICHA_BASE_URL",
        "PJUPI_PURE_API_BASE_URL",
        "PJUPI_PURE_API_KEY",
    ] {
        if let Ok(value) = env::var(key) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                values.insert(key.to_string(), trimmed.to_string());
            }
        }
    }
}

fn merge_env_file(values: &mut HashMap<String, String>, path: &Path) -> Result<(), AppError> {
    let content = fs::read_to_string(path).map_err(|error| {
        AppError::ConfigurationError(format!("No se pudo leer el archivo de configuración {:?}: {}", path, error))
    })?;

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((raw_key, raw_value)) = line.split_once('=') else {
            continue;
        };

        let key = raw_key.trim();
        if key.is_empty() {
            continue;
        }

        let value = raw_value.trim().trim_matches('"').trim_matches('\'').trim().to_string();
        values.insert(key.to_string(), value);
    }

    Ok(())
}