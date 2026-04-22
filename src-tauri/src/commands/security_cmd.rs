/// Security and configuration status commands
/// Provides information about application security and configuration state

use crate::config::DatabaseBackend;
use crate::state::AppState;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SecurityStatus {
    pub database_backend: String,
    pub mongodb_configured: bool,
    pub sqlite_initialized: bool,
    pub security_recommendations: Vec<String>,
}

#[tauri::command]
pub async fn get_security_status(state: tauri::State<'_, AppState>) -> Result<SecurityStatus, String> {
    let db_backend = state.primary_backend;
    let mongodb_configured = state.mongo.is_some();
    let sqlite_initialized = state.sqlite.is_some();

    let mut recommendations = Vec::new();

    // Check if MongoDB is required but not configured
    if matches!(db_backend, DatabaseBackend::MongoDb) && !mongodb_configured {
        recommendations.push(
            "⚠️ MongoDB está configurado como backend primario pero no está disponible. Verifique la configuración de PJUPI_MONGODB_URI.".to_string()
        );
    }

    // Warn if running in SQLite-only mode
    if matches!(db_backend, DatabaseBackend::Sqlite) {
        recommendations.push(
            "ℹ️ Ejecutando en modo SQLite local. Los cambios offline no se sincronizarán con MongoDB.".to_string()
        );
    }

    // Check if both backends are available (recommended setup)
    if mongodb_configured && sqlite_initialized {
        recommendations.push(
            "✓ Configuración óptima: MongoDB primario + SQLite local para sincronización offline.".to_string()
        );
    }

    let backend_name = match db_backend {
        DatabaseBackend::MongoDb => "MongoDB".to_string(),
        DatabaseBackend::Sqlite => "SQLite".to_string(),
    };

    Ok(SecurityStatus {
        database_backend: backend_name,
        mongodb_configured,
        sqlite_initialized,
        security_recommendations: recommendations,
    })
}

#[derive(Serialize, Debug)]
pub struct ConfigurationGuide {
    pub title: String,
    pub steps: Vec<ConfigurationStep>,
}

#[derive(Serialize, Debug)]
pub struct ConfigurationStep {
    pub step_number: u32,
    pub title: String,
    pub description: String,
    pub example: Option<String>,
}

#[tauri::command]
pub async fn get_setup_guide() -> Result<ConfigurationGuide, String> {
    let steps = vec![
        ConfigurationStep {
            step_number: 1,
            title: "Crear archivo de configuración".to_string(),
            description: "El archivo de configuración se encuentra en:\n- Windows: %APPDATA%/pjupi/pjupi.env\n- macOS: ~/Library/Application Support/pjupi/pjupi.env\n- Linux: ~/.local/share/pjupi/pjupi.env".to_string(),
            example: Some("# pjupi.env\nPJUPI_DB_BACKEND=mongodb\nPJUPI_MONGODB_URI=mongodb://localhost:27017\nPJUPI_MONGODB_DB=pjupi".to_string()),
        },
        ConfigurationStep {
            step_number: 2,
            title: "Configurar base de datos".to_string(),
            description: "Seleccione entre:\n- mongodb: Para sincronización empresarial (recomendado)\n- sqlite: Para uso local sin servidor".to_string(),
            example: None,
        },
        ConfigurationStep {
            step_number: 3,
            title: "Configurar API externos (opcional)".to_string(),
            description: "Agregue tokens de RENIEC si desea consultar datos del DNI:\nPJUPI_RENIEC_TOKEN=su_token_aqui".to_string(),
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

#[derive(Serialize, Debug)]
pub struct SecurityRecommendations {
    pub recommendations: Vec<SecurityRecommendation>,
}

#[derive(Serialize, Debug)]
pub struct SecurityRecommendation {
    pub category: String,
    pub title: String,
    pub description: String,
    pub priority: String, // "high", "medium", "low"
}

#[tauri::command]
pub async fn get_security_recommendations() -> Result<SecurityRecommendations, String> {
    let recommendations = vec![
        SecurityRecommendation {
            category: "Configuración".to_string(),
            title: "Proteger archivo de configuración".to_string(),
            description: "El archivo pjupi.env contiene credenciales sensibles. Asegúrese de que solo el usuario actual pueda acceder.\nEn Linux/macOS: chmod 600 ~/.local/share/pjupi/pjupi.env".to_string(),
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
            description: "Los tokens de API (RENIEC, RENACYT) deben rotarse regularmente según la política de seguridad del proveedor.".to_string(),
            priority: "medium".to_string(),
        },
        SecurityRecommendation {
            category: "Base de datos".to_string(),
            title: "Hacer backup regularmente".to_string(),
            description: "SQLite almacena cambios offline localmente. Haga backup de: ~/.local/share/pjupi/database.db".to_string(),
            priority: "medium".to_string(),
        },
        SecurityRecommendation {
            category: "Red".to_string(),
            title: "Monitorear sincronización".to_string(),
            description: "Revise regularmente la tabla sync_conflicts para detectar problemas de sincronización offline/online.".to_string(),
            priority: "low".to_string(),
        },
    ];

    Ok(SecurityRecommendations { recommendations })
}
