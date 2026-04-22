/// Setup wizard module for initial application configuration
/// 
/// Provides a guided setup process for:
/// - Selecting database backend (MongoDB or SQLite-only)
/// - Configuring MongoDB connection
/// - Setting up API credentials
/// - Initializing first admin user

use std::collections::HashMap;

#[allow(dead_code)]
use crate::config::DatabaseConfig;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SetupWizardState {
    pub backend_choice: Option<String>, // "mongodb" or "sqlite"
    pub mongodb_uri: Option<String>,
    pub mongodb_db_name: String,
    pub reniec_token: Option<String>,
    pub renacyt_api_configured: bool,
}

impl Default for SetupWizardState {
    fn default() -> Self {
        Self {
            backend_choice: None,
            mongodb_uri: None,
            mongodb_db_name: "pjupi".to_string(),
            reniec_token: None,
            renacyt_api_configured: false,
        }
    }
}

#[allow(dead_code)]
impl SetupWizardState {
    /// Generate environment variable content from setup wizard state
    pub fn to_env_content(&self) -> String {
        let mut content = String::new();

        // Backend configuration
        if let Some(backend) = &self.backend_choice {
            content.push_str(&format!("# Backend Selection (mongodb or sqlite)\n"));
            content.push_str(&format!("PJUPI_DB_BACKEND={}\n\n", backend));
        }

        // MongoDB configuration
        if let Some(uri) = &self.mongodb_uri {
            content.push_str(&format!("# MongoDB Configuration\n"));
            content.push_str(&format!("PJUPI_MONGODB_URI={}\n", uri));
            content.push_str(&format!("PJUPI_MONGODB_DB={}\n\n", self.mongodb_db_name));
        }

        // RENIEC API Configuration (optional)
        if let Some(token) = &self.reniec_token {
            content.push_str(&format!("# RENIEC API Configuration\n"));
            content.push_str(&format!("# Do not share this token publicly\n"));
            content.push_str(&format!("PJUPI_RENIEC_TOKEN={}\n\n", token));
        }

        // Default API endpoints (can be customized)
        content.push_str(&format!("# RENACYT API Configuration (default values)\n"));
        content.push_str(&format!("PJUPI_RENACYT_API_BASE_URL=https://renacyt.concytec.gob.pe/renacyt-backend\n"));
        content.push_str(&format!("PJUPI_RENACYT_ACTO_VERSION=2021\n"));
        content.push_str(&format!("PJUPI_RENACYT_FICHA_BASE_URL=https://servicio-renacyt.concytec.gob.pe/ficha-renacyt/\n\n"));

        content.push_str(&format!("# RENIEC API Configuration (optional)\n"));
        content.push_str(&format!("# PJUPI_RENIEC_API_BASE_URL=https://api.decolecta.com/v1\n"));
        content.push_str(&format!("# PJUPI_RENIEC_TOKEN=your_token_here\n"));

        content
    }

    /// Convert wizard state to HashMap for DatabaseConfig
    pub fn to_config_values(&self) -> HashMap<String, String> {
        let mut values = HashMap::new();

        if let Some(backend) = &self.backend_choice {
            values.insert("PJUPI_DB_BACKEND".to_string(), backend.clone());
        }

        if let Some(uri) = &self.mongodb_uri {
            values.insert("PJUPI_MONGODB_URI".to_string(), uri.clone());
            values.insert("PJUPI_MONGODB_DB".to_string(), self.mongodb_db_name.clone());
        }

        if let Some(token) = &self.reniec_token {
            values.insert("PJUPI_RENIEC_TOKEN".to_string(), token.clone());
        }

        values
    }
}

/// Provides user-friendly wizard step descriptions and validation hints
#[allow(dead_code)]
pub struct SetupWizardSteps;

#[allow(dead_code)]
impl SetupWizardSteps {
    pub const STEP_1_BACKEND_CHOICE: &'static str = 
        "Seleccione el backend de base de datos:\n\
         1. MongoDB (recomendado para aplicaciones empresariales con sincronización offline)\n\
         2. SQLite (configuración local, no requiere servidor externo)\n";

    pub const STEP_2_MONGODB_URI: &'static str =
        "Ingrese la URI de conexión a MongoDB:\n\
         Formato: mongodb://[usuario:contraseña@]host[:puerto][/base_de_datos]\n\
         Ejemplo: mongodb://localhost:27017/pjupi\n";

    pub const STEP_3_RENIEC_TOKEN: &'static str =
        "Ingrese el token de API para RENIEC (opcional):\n\
         Si no tiene token, puede dejar vacío y configurarlo después.\n\
         El token será almacenado en el archivo de configuración local.\n";

    pub const STEP_4_REVIEW: &'static str =
        "Revise la configuración generada. Los cambios se guardarán en:\n\
         ~/.pjupi/pjupi.env\n";
}

/// Validates user input during setup wizard
#[allow(dead_code)]
pub struct SetupWizardValidator;

#[allow(dead_code)]
impl SetupWizardValidator {
    pub fn validate_backend_choice(input: &str) -> Result<String, String> {
        match input.trim().to_lowercase().as_str() {
            "1" | "mongodb" | "mongo" => Ok("mongodb".to_string()),
            "2" | "sqlite" | "local" => Ok("sqlite".to_string()),
            _ => Err("Opción inválida. Seleccione 1 (MongoDB) o 2 (SQLite).".to_string()),
        }
    }

    pub fn validate_mongodb_uri(uri: &str) -> Result<String, String> {
        let trimmed = uri.trim();
        
        if trimmed.is_empty() {
            return Err("URI no puede estar vacía.".to_string());
        }

        if !trimmed.starts_with("mongodb://") && !trimmed.starts_with("mongodb+srv://") {
            return Err("URI debe comenzar con mongodb:// o mongodb+srv://".to_string());
        }

        Ok(trimmed.to_string())
    }

    pub fn validate_database_name(name: &str) -> Result<String, String> {
        let trimmed = name.trim();

        if trimmed.is_empty() {
            return Ok("pjupi".to_string()); // Use default
        }

        if !trimmed.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err("Nombre de base de datos contiene caracteres inválidos.".to_string());
        }

        Ok(trimmed.to_string())
    }

    pub fn validate_api_token(token: &str) -> Result<Option<String>, String> {
        let trimmed = token.trim();

        if trimmed.is_empty() {
            return Ok(None); // Token es opcional
        }

        if trimmed.len() < 10 {
            return Err("Token parece muy corto. Verifique que sea correcto.".to_string());
        }

        Ok(Some(trimmed.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wizard_state_to_env() {
        let mut state = SetupWizardState::default();
        state.backend_choice = Some("mongodb".to_string());
        state.mongodb_uri = Some("mongodb://localhost:27017".to_string());

        let env = state.to_env_content();
        assert!(env.contains("PJUPI_DB_BACKEND=mongodb"));
        assert!(env.contains("PJUPI_MONGODB_URI=mongodb://localhost:27017"));
    }

    #[test]
    fn test_validate_backend_choice() {
        assert_eq!(
            SetupWizardValidator::validate_backend_choice("1"),
            Ok("mongodb".to_string())
        );
        assert_eq!(
            SetupWizardValidator::validate_backend_choice("mongodb"),
            Ok("mongodb".to_string())
        );
        assert!(SetupWizardValidator::validate_backend_choice("3").is_err());
    }

    #[test]
    fn test_validate_mongodb_uri() {
        assert!(SetupWizardValidator::validate_mongodb_uri("mongodb://localhost").is_ok());
        assert!(SetupWizardValidator::validate_mongodb_uri("invalid").is_err());
        assert!(SetupWizardValidator::validate_mongodb_uri("").is_err());
    }
}
