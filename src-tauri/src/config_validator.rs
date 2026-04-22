/// Configuration validation module for security and correctness checks
/// 
/// This module provides:
/// - Configuration completeness validation
/// - Security checks (credentials, permissions)
/// - User-friendly error messages for troubleshooting

use std::path::Path;

use crate::config::DatabaseConfig;
use crate::error::AppError;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ValidationError {
    MissingMongoDbUri,
    InvalidSqlitePath,
    InvalidMongoDbUri(String),
    SqlitePermissionDenied(String),
    MissingCriticalConfig(String),
    InconsistentBackendConfig,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::MissingMongoDbUri => {
                write!(f, "MongoDB URI no configurada. Configure PJUPI_MONGODB_URI en su archivo .env o en las variables de entorno.")
            }
            ValidationError::InvalidSqlitePath => {
                write!(f, "Ruta de SQLite es inválida o no es accesible.")
            }
            ValidationError::InvalidMongoDbUri(reason) => {
                write!(f, "URI de MongoDB es inválida: {}. Formato esperado: mongodb://[usuario:contraseña@]host[:puerto][/base_de_datos]", reason)
            }
            ValidationError::SqlitePermissionDenied(path) => {
                write!(f, "Permiso denegado para acceder a la base de datos SQLite en: {}. Verifique los permisos del directorio.", path)
            }
            ValidationError::MissingCriticalConfig(msg) => {
                write!(f, "Configuración crítica faltante: {}", msg)
            }
            ValidationError::InconsistentBackendConfig => {
                write!(f, "Configuración de backend inconsistente. Si PJUPI_DB_BACKEND=mongodb, debe proporcionar PJUPI_MONGODB_URI.")
            }
        }
    }
}

impl From<ValidationError> for AppError {
    fn from(err: ValidationError) -> Self {
        AppError::ConfigurationError(err.to_string())
    }
}

/// Validates the database configuration for consistency and security
pub fn validate_database_config(config: &DatabaseConfig) -> Result<(), ValidationError> {
    // Check if backend is MongoDB but URI is missing
    if config.requires_mongodb() && config.mongodb_uri.is_none() {
        return Err(ValidationError::MissingMongoDbUri);
    }

    // Validate MongoDB URI format if provided
    if let Some(uri) = &config.mongodb_uri {
        validate_mongodb_uri(uri)?;
    }

    // Validate SQLite path
    validate_sqlite_url(&config.sqlite_url)?;

    // Check for inconsistent configuration
    if config.requires_mongodb() && config.mongodb_uri.is_none() {
        return Err(ValidationError::InconsistentBackendConfig);
    }

    Ok(())
}

/// Validates MongoDB URI format and basic structure
fn validate_mongodb_uri(uri: &str) -> Result<(), ValidationError> {
    if uri.is_empty() {
        return Err(ValidationError::InvalidMongoDbUri("URI is empty".to_string()));
    }

    if !uri.starts_with("mongodb://") && !uri.starts_with("mongodb+srv://") {
        return Err(ValidationError::InvalidMongoDbUri(
            "URI must start with mongodb:// or mongodb+srv://".to_string(),
        ));
    }

    // Basic check that URI contains a host after the scheme
    let after_scheme = if uri.starts_with("mongodb+srv://") {
        &uri[14..]
    } else {
        &uri[10..]
    };

    if after_scheme.is_empty() {
        return Err(ValidationError::InvalidMongoDbUri(
            "No host specified after scheme".to_string(),
        ));
    }

    Ok(())
}

/// Validates SQLite URL format and path accessibility
fn validate_sqlite_url(url: &str) -> Result<(), ValidationError> {
    if !url.starts_with("sqlite://") {
        return Err(ValidationError::InvalidSqlitePath);
    }

    // Extract path from URL
    let path_str = if url.starts_with("sqlite:///") {
        // sqlite:///C:/path/to/db.db or sqlite:////unix/path/to/db.db
        &url[10..]
    } else {
        // sqlite://path (relative or UNC path)
        &url[9..]
    };

    if path_str.is_empty() {
        return Err(ValidationError::InvalidSqlitePath);
    }

    let path = Path::new(path_str);

    // Check if parent directory is accessible
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            // It's OK if parent doesn't exist - SQLite will create it
            // But we should verify write permissions if it does exist
        } else if parent.exists() {
            // Parent exists, check if we can access it
            match std::fs::metadata(parent) {
                Ok(_) => {
                    // We have read access
                }
                Err(e) => {
                    return Err(ValidationError::SqlitePermissionDenied(format!(
                        "{}: {}",
                        parent.display(),
                        e
                    )));
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_mongodb_uri_valid() {
        assert!(validate_mongodb_uri("mongodb://localhost:27017").is_ok());
        assert!(validate_mongodb_uri("mongodb+srv://user:pass@cluster.mongodb.net/db").is_ok());
    }

    #[test]
    fn test_validate_mongodb_uri_invalid() {
        assert!(validate_mongodb_uri("").is_err());
        assert!(validate_mongodb_uri("http://localhost:27017").is_err());
        assert!(validate_mongodb_uri("mongodb://").is_err());
    }
}
