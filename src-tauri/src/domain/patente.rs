use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn current_timestamp_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Patente {
    #[serde(rename = "_id")]
    pub id: String,
    pub id_patente: String,
    pub proyecto_id: Option<String>,
    pub docente_id: Option<String>,
    pub titulo: String,
    pub numero_patente: Option<String>,
    /// "invencion" | "modelo_utilidad" | "diseno_industrial"
    pub tipo: Option<String>,
    pub estado: Option<String>,
    pub fecha_solicitud: Option<i64>,
    pub fecha_concesion: Option<i64>,
    pub pais: Option<String>,
    pub entidad_concedente: Option<String>,
    pub descripcion: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

impl Patente {
    pub fn new(request: CreatePatenteRequest) -> Self {
        let now = current_timestamp_ms();
        let id = Uuid::new_v4().to_string();
        Self {
            id: id.clone(),
            id_patente: id,
            proyecto_id: request.proyecto_id,
            docente_id: request.docente_id,
            titulo: request.titulo,
            numero_patente: request.numero_patente,
            tipo: request.tipo,
            estado: request.estado,
            fecha_solicitud: request.fecha_solicitud,
            fecha_concesion: request.fecha_concesion,
            pais: request.pais,
            entidad_concedente: request.entidad_concedente,
            descripcion: request.descripcion,
            created_at: Some(now),
            updated_at: Some(now),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatePatenteRequest {
    pub proyecto_id: Option<String>,
    pub docente_id: Option<String>,
    pub titulo: String,
    pub numero_patente: Option<String>,
    pub tipo: Option<String>,
    pub estado: Option<String>,
    pub fecha_solicitud: Option<i64>,
    pub fecha_concesion: Option<i64>,
    pub pais: Option<String>,
    pub entidad_concedente: Option<String>,
    pub descripcion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdatePatenteRequest {
    pub titulo: Option<String>,
    pub numero_patente: Option<String>,
    pub tipo: Option<String>,
    pub estado: Option<String>,
    pub fecha_solicitud: Option<i64>,
    pub fecha_concesion: Option<i64>,
    pub pais: Option<String>,
    pub entidad_concedente: Option<String>,
    pub descripcion: Option<String>,
}
