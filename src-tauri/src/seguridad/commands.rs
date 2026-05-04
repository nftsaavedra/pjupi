use crate::shared::error::AppError;
use crate::shared::state::AppState;
use crate::seguridad::models::{
    ConfigurationGuide, ConfigurationStep, SecurityRecommendation, SecurityRecommendations, SecurityStatus,
};

#[tauri::command]
pub async fn get_security_status(state: tauri::State<'_, AppState>) -> Result<SecurityStatus, AppError> {
    let mongodb_configured = state.mongo.is_some();

    let mut recommendations = Vec::new();

    if !mongodb_configured {
        recommendations.push(
            "⚠️ MongoDB no está disponible. Verifique la configuración de PJUPI_MONGODB_URI.".to_string()
        );
    }

    Ok(SecurityStatus {
        database_backend: "MongoDB".to_string(),
        mongodb_configured,
        security_recommendations: recommendations,
    })
}

#[tauri::command]
pub async fn get_setup_guide() -> Result<ConfigurationGuide, AppError> {
    let steps = vec![
        ConfigurationStep {
            step_number: 1,
            title: "Crear archivo de configuración".to_string(),
            description: "El archivo de configuración se encuentra en:\n- Windows: %APPDATA%/com.upic.pjupi/pjupi.config.json\n- macOS: ~/Library/Application Support/com.upic.pjupi/pjupi.config.json\n- Linux: ~/.local/share/com.upic.pjupi/pjupi.config.json".to_string(),
            example: Some("{\n  \"database\": {\n    \"mongodbUri\": \"mongodb://localhost:27017\",\n    \"mongodbDb\": \"pjupi\"\n  }\n}".to_string()),
        },
        ConfigurationStep {
            step_number: 2,
            title: "Configurar base de datos".to_string(),
            description: "MongoDB es el backend obligatorio en esta versión. Configure la URI y el nombre de la base en pjupi.config.json.".to_string(),
            example: None,
        },
        ConfigurationStep {
            step_number: 3,
            title: "Configurar API externos (opcional)".to_string(),
            description: "Agregue credenciales externas según funcionalidades usadas:\nPJUPI_RENIEC_TOKEN=su_token_aqui\nPJUPI_PURE_API_KEY=su_api_key_pure".to_string(),
            example: None,
        },
        ConfigurationStep {
            step_number: 4,
            title: "Reiniciar la aplicación".to_string(),
            description: "Los cambios se aplicarán al reiniciar PJUPI.".to_string(),
            example: None,
        },
    ];

    Ok(ConfigurationGuide {
        title: "Guía de Configuración de PJUPI".to_string(),
        steps,
    })
}

#[tauri::command]
pub async fn get_security_recommendations() -> Result<SecurityRecommendations, AppError> {
    let recommendations = vec![
        SecurityRecommendation {
            category: "Configuración".to_string(),
            title: "Proteger archivo de configuración".to_string(),
            description: "El archivo pjupi.config.json contiene credenciales sensibles. Asegúrese de que solo el usuario actual pueda acceder.\nEn Linux/macOS: chmod 600 ~/.local/share/com.upic.pjupi/pjupi.config.json".to_string(),
            priority: "high".to_string(),
        },
        SecurityRecommendation {
            category: "MongoDB".to_string(),
            title: "Usar conexión segura (SSL/TLS)".to_string(),
            description: "Para MongoDB en producción, use mongodb+srv:// con SSL/TLS habilitado:\nmongodb+srv://usuario:contraseña@cluster.mongodb.net/pjupi".to_string(),
            priority: "high".to_string(),
        },
        SecurityRecommendation {
            category: "API".to_string(),
            title: "Mantener tokens actualizados".to_string(),
            description: "Los secretos de API (RENIEC y Pure) deben rotarse regularmente según la política de seguridad del proveedor.".to_string(),
            priority: "medium".to_string(),
        },
        SecurityRecommendation {
            category: "Base de datos".to_string(),
            title: "Hacer backup regularmente".to_string(),
            description: "Configure backups automáticos en MongoDB (snapshots y/o dumps) según su RPO/RTO.".to_string(),
            priority: "medium".to_string(),
        },
        SecurityRecommendation {
            category: "Red".to_string(),
            title: "Monitorear disponibilidad de MongoDB".to_string(),
            description: "Implemente alertas de latencia/conectividad para evitar interrupciones de operaciones en tiempo real.".to_string(),
            priority: "low".to_string(),
        },
    ];

    Ok(SecurityRecommendations { recommendations })
}
