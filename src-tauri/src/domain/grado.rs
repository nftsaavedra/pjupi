use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct GradoAcademico {
    pub id_grado: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub activo: i64,
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
        Self {
            id_grado: Uuid::new_v4().to_string(),
            nombre: request.nombre,
            descripcion: request.descripcion,
            activo: 1,
        }
    }
}