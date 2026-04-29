/// Setup wizard module for initial application configuration
/// 
/// Provides a guided setup process for:
/// - Configuring MongoDB connection
/// - Setting up API credentials
/// - Initializing first admin user

use std::collections::HashMap;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SetupWizardState {
    pub mongodb_uri: Option<String>,
    pub mongodb_db_name: String,
    pub reniec_token: Option<String>,
    pub pure_api_key: Option<String>,
    pub renacyt_api_configured: bool,
}

impl Default for SetupWizardState {
    fn default() -> Self {
        Self {
            mongodb_uri: None,
            mongodb_db_name: "pjupi".to_string(),
            reniec_token: None,
            pure_api_key: None,
            renacyt_api_configured: false,
        }
    }
}

#[allow(dead_code)]
impl SetupWizardState {
    /// Generate JSON configuration content from setup wizard state
    pub fn to_json_content(&self) -> String {
        let mongodb_uri = self.mongodb_uri.clone().unwrap_or_default();
        let reniec_token = self.reniec_token.clone().unwrap_or_default();
        let pure_api_key = self.pure_api_key.clone().unwrap_or_default();

        format!(
            "{{\n  \"database\": {{\n    \"mongodbUri\": \"{}\",\n    \"mongodbDb\": \"{}\"\n  }},\n  \"reniec\": {{\n    \"apiBaseUrl\": \"https://api.decolecta.com/v1\",\n    \"token\": \"{}\"\n  }},\n  \"renacyt\": {{\n    \"apiBaseUrl\": \"https://renacyt.concytec.gob.pe/renacyt-backend\",\n    \"actoVersion\": \"2021\",\n    \"fichaBaseUrl\": \"https://servicio-renacyt.concytec.gob.pe/ficha-renacyt/\"\n  }},\n  \"pure\": {{\n    \"apiBaseUrl\": \"https://pure.unf.edu.pe/ws/api\",\n    \"apiKey\": \"{}\"\n  }}\n}}\n",
            escape_json_string(&mongodb_uri),
            escape_json_string(&self.mongodb_db_name),
            escape_json_string(&reniec_token),
            escape_json_string(&pure_api_key)
        )
    }

    /// Convert wizard state to HashMap for DatabaseConfig
    pub fn to_config_values(&self) -> HashMap<String, String> {
        let mut values = HashMap::new();

        values.insert("PJUPI_DB_BACKEND".to_string(), "mongodb".to_string());

        if let Some(uri) = &self.mongodb_uri {
            values.insert("PJUPI_MONGODB_URI".to_string(), uri.clone());
            values.insert("PJUPI_MONGODB_DB".to_string(), self.mongodb_db_name.clone());
        }

        if let Some(token) = &self.reniec_token {
            values.insert("PJUPI_RENIEC_TOKEN".to_string(), token.clone());
        }

        if let Some(api_key) = &self.pure_api_key {
            values.insert("PJUPI_PURE_API_KEY".to_string(), api_key.clone());
        }

        values
    }
}

fn escape_json_string(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
}

/// Provides user-friendly wizard step descriptions and validation hints
#[allow(dead_code)]
pub struct SetupWizardSteps;

#[allow(dead_code)]
impl SetupWizardSteps {
    pub const STEP_1_BACKEND_CHOICE: &'static str = 
        "El backend de base de datos es MongoDB por defecto en esta version.\n\
         Solo necesita configurar la URI y la base de datos objetivo.\n";

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
         ~/.pjupi/pjupi.config.json\n";
}

/// Validates user input during setup wizard
#[allow(dead_code)]
pub struct SetupWizardValidator;

#[allow(dead_code)]
impl SetupWizardValidator {
    pub fn validate_backend_choice(input: &str) -> Result<String, String> {
        let normalized = input.trim().to_lowercase();
        if normalized.is_empty() || normalized == "1" || normalized == "mongodb" || normalized == "mongo" {
            Ok("mongodb".to_string())
        } else {
            Err("Solo MongoDB esta soportado en esta version. Use 'mongodb'.".to_string())
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
        state.mongodb_uri = Some("mongodb://localhost:27017".to_string());

        let json = state.to_json_content();
        assert!(json.contains("\"mongodbUri\": \"mongodb://localhost:27017\""));
        assert!(json.contains("\"mongodbDb\": \"pjupi\""));
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
        assert!(SetupWizardValidator::validate_backend_choice("sqlite").is_err());
    }

    #[test]
    fn test_validate_mongodb_uri() {
        assert!(SetupWizardValidator::validate_mongodb_uri("mongodb://localhost").is_ok());
        assert!(SetupWizardValidator::validate_mongodb_uri("invalid").is_err());
        assert!(SetupWizardValidator::validate_mongodb_uri("").is_err());
    }
}
