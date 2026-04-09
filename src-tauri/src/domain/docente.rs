use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Docente {
    pub id_docente: String,
    pub dni: String,
    pub id_grado: String,
    pub nombres_apellidos: String,
    pub nombres: Option<String>,
    pub apellido_paterno: Option<String>,
    pub apellido_materno: Option<String>,
    pub activo: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateDocenteRequest {
    pub dni: String,
    pub id_grado: String,
    pub nombres: String,
    pub apellido_paterno: String,
    pub apellido_materno: Option<String>,
}

// New: Docente with detail data for reports
#[derive(Debug, Serialize, FromRow)]
pub struct DocenteDetalle {
    pub id_docente: String,
    pub dni: String,
    pub nombres_apellidos: String,
    pub nombres: Option<String>,
    pub apellido_paterno: Option<String>,
    pub apellido_materno: Option<String>,
    pub grado: String,
    pub cantidad_proyectos: i64,
    pub proyectos: Option<String>, // JSON array of project titles, comma-separated
    pub activo: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReniecDniLookupResult {
    pub first_name: String,
    pub first_last_name: String,
    pub second_last_name: String,
    pub full_name: String,
    pub document_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EliminarDocenteResultado {
    pub accion: String, // "desactivado"
    pub mensaje: String,
}

impl Docente {
    pub fn new(request: CreateDocenteRequest) -> Self {
        let apellido_materno = request
            .apellido_materno
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let nombres = request.nombres.trim().to_string();
        let apellido_paterno = request.apellido_paterno.trim().to_string();
        let nombres_apellidos = [
            Some(nombres.clone()),
            Some(apellido_paterno.clone()),
            apellido_materno.clone(),
        ]
        .into_iter()
        .flatten()
        .filter(|value| !value.trim().is_empty())
        .collect::<Vec<_>>()
        .join(" ");

        Self {
            id_docente: Uuid::new_v4().to_string(),
            dni: request.dni,
            id_grado: request.id_grado,
            nombres_apellidos,
            nombres: Some(nombres),
            apellido_paterno: Some(apellido_paterno),
            apellido_materno,
            activo: 1,
        }
    }
}