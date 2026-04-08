use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Docente {
    pub id_docente: String,
    pub dni: String,
    pub id_grado: String,
    pub nombres_apellidos: String,
    pub activo: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateDocenteRequest {
    pub dni: String,
    pub id_grado: String,
    pub nombres_apellidos: String,
}

// New: Docente with detail data for reports
#[derive(Debug, Serialize, FromRow)]
pub struct DocenteDetalle {
    pub id_docente: String,
    pub dni: String,
    pub nombres_apellidos: String,
    pub grado: String,
    pub cantidad_proyectos: i64,
    pub proyectos: Option<String>, // JSON array of project titles, comma-separated
    pub activo: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EliminarDocenteResultado {
    pub accion: String, // "desactivado"
    pub mensaje: String,
}

impl Docente {
    pub fn new(request: CreateDocenteRequest) -> Self {
        Self {
            id_docente: Uuid::new_v4().to_string(),
            dni: request.dni,
            id_grado: request.id_grado,
            nombres_apellidos: request.nombres_apellidos,
            activo: 1,
        }
    }
}