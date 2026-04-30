/// Configuration validation module for security and correctness checks
///
/// Validates database configuration completeness and URI format, returning
/// user-friendly `AppError::ConfigurationError` messages on failure.

use crate::config::DatabaseConfig;
use crate::error::AppError;

/// Validates the database configuration for consistency and correctness.
pub fn validate_database_config(config: &DatabaseConfig) -> Result<(), AppError> {
    if config.requires_mongodb() && config.mongodb_uri.is_none() {
        return Err(AppError::ConfigurationError(
            "MongoDB URI no configurada. Configure PJUPI_MONGODB_URI en su archivo de configuración o variable de entorno.".to_string(),
        ));
    }

    if let Some(uri) = &config.mongodb_uri {
        validate_mongodb_uri(uri)?;
    }

    Ok(())
}

fn validate_mongodb_uri(uri: &str) -> Result<(), AppError> {
    if uri.is_empty() {
        return Err(AppError::ConfigurationError(
            "La URI de MongoDB no puede estar vacía.".to_string(),
        ));
    }

    if !uri.starts_with("mongodb://") && !uri.starts_with("mongodb+srv://") {
        return Err(AppError::ConfigurationError(
            "La URI de MongoDB debe comenzar con mongodb:// o mongodb+srv://".to_string(),
        ));
    }

    let after_scheme = if uri.starts_with("mongodb+srv://") {
        &uri[14..]
    } else {
        &uri[10..]
    };

    if after_scheme.is_empty() {
        return Err(AppError::ConfigurationError(
            "La URI de MongoDB no especifica un host válido.".to_string(),
        ));
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
