use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProyectoParticipanteResumen {
    pub nombre: String,
    pub grado: String,
    pub renacyt_nivel: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Proyecto {
    pub id_proyecto: String,
    pub titulo_proyecto: String,
    pub activo: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateProyectoRequest {
    pub titulo_proyecto: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProyectoConParticipantesRequest {
    pub titulo_proyecto: String,
    pub docentes_ids: Vec<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ProyectoDetalle {
    pub id_proyecto: String,
    pub titulo_proyecto: String,
    pub cantidad_docentes: i64,
    pub docentes: Option<String>,
    pub participantes_json: Option<String>,
    pub activo: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EliminarProyectoResultado {
    pub accion: String, // "desactivado"
    pub mensaje: String,
}

// NEW: Enhanced export data grouped by docente
#[derive(Debug, Serialize, FromRow)]
pub struct ExportDataConProjectos {
    pub docente: String,
    pub dni: String,
    pub grado: String,
    pub renacyt_nivel: String,
    pub cantidad_proyectos: i64,
    pub proyectos: Option<String>, // comma-separated project titles
}

impl Proyecto {
    pub fn new(request: CreateProyectoRequest) -> Self {
        Self {
            id_proyecto: Uuid::new_v4().to_string(),
            titulo_proyecto: request.titulo_proyecto,
            activo: 1,
        }
    }
}