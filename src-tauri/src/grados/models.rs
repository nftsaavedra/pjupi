use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GradoAcademico {
    pub id_grado: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub activo: i64,
    #[serde(default)]
    pub updated_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EliminarGradoResultado {
    pub accion: String, // "eliminado" | "desactivado"
    pub mensaje: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateGradoRequest {
    pub nombre: String,
    pub descripcion: Option<String>,
}

impl GradoAcademico {
    pub fn new(request: CreateGradoRequest) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_millis() as i64)
            .unwrap_or_default();
        
        Self {
            id_grado: Uuid::new_v4().to_string(),
            nombre: request.nombre,
            descripcion: request.descripcion,
            activo: 1,
            updated_at: Some(now),
        }
    }
}
