#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Patente derivada de un proyecto o de la actividad investigadora de un docente.
/// La asociación a proyecto es opcional (0..N por proyecto).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Patente {
    pub id_patente: String,
    /// Vínculo opcional a un proyecto institucional.
    #[serde(default)]
    pub proyecto_id: Option<String>,
    /// Docente titular o responsable (opcional cuando se asocia sólo a proyecto).
    #[serde(default)]
    pub docente_id: Option<String>,
    pub titulo: String,
    #[serde(default)]
    pub numero_patente: Option<String>,
    /// Ej.: "invencion", "modelo_utilidad", "diseno_industrial"
    #[serde(default)]
    pub tipo: Option<String>,
    #[serde(default)]
    pub fecha_solicitud: Option<i64>,
    #[serde(default)]
    pub fecha_concesion: Option<i64>,
    #[serde(default)]
    pub pais: Option<String>,
    #[serde(default)]
    pub entidad_concedente: Option<String>,
    #[serde(default)]
    pub descripcion: Option<String>,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl Patente {
    pub fn new(titulo: String, now_ms: i64) -> Self {
        Self {
            id_patente: Uuid::new_v4().to_string(),
            proyecto_id: None,
            docente_id: None,
            titulo,
            numero_patente: None,
            tipo: None,
            fecha_solicitud: None,
            fecha_concesion: None,
            pais: None,
            entidad_concedente: None,
            descripcion: None,
            created_at: Some(now_ms),
            updated_at: Some(now_ms),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreatePatenteRequest {
    pub proyecto_id: Option<String>,
    pub docente_id: Option<String>,
    pub titulo: String,
    pub numero_patente: Option<String>,
    pub tipo: Option<String>,
    pub fecha_solicitud: Option<i64>,
    pub fecha_concesion: Option<i64>,
    pub pais: Option<String>,
    pub entidad_concedente: Option<String>,
    pub descripcion: Option<String>,
}
