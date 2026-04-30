use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn current_timestamp_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Financiamiento {
    #[serde(rename = "_id")]
    pub id: String,
    pub id_financiamiento: String,
    pub proyecto_id: Option<String>,
    pub entidad_financiadora: String,
    /// "nacional" | "internacional" | "propio" | "concursable"
    pub tipo: Option<String>,
    pub monto: Option<f64>,
    pub moneda: Option<String>,
    pub fecha_inicio: Option<i64>,
    pub fecha_fin: Option<i64>,
    pub descripcion: Option<String>,
    pub estado_financiero: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

impl Financiamiento {
    pub fn new(request: CreateFinanciamientoRequest) -> Self {
        let now = current_timestamp_ms();
        let id = Uuid::new_v4().to_string();
        Self {
            id: id.clone(),
            id_financiamiento: id,
            proyecto_id: request.proyecto_id,
            entidad_financiadora: request.entidad_financiadora,
            tipo: request.tipo,
            monto: request.monto,
            moneda: request.moneda,
            fecha_inicio: request.fecha_inicio,
            fecha_fin: request.fecha_fin,
            descripcion: request.descripcion,
            estado_financiero: request.estado_financiero,
            created_at: Some(now),
            updated_at: Some(now),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateFinanciamientoRequest {
    pub proyecto_id: Option<String>,
    pub entidad_financiadora: String,
    pub tipo: Option<String>,
    pub monto: Option<f64>,
    pub moneda: Option<String>,
    pub fecha_inicio: Option<i64>,
    pub fecha_fin: Option<i64>,
    pub descripcion: Option<String>,
    pub estado_financiero: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateFinanciamientoRequest {
    pub entidad_financiadora: Option<String>,
    pub tipo: Option<String>,
    pub monto: Option<f64>,
    pub moneda: Option<String>,
    pub fecha_inicio: Option<i64>,
    pub fecha_fin: Option<i64>,
    pub descripcion: Option<String>,
    pub estado_financiero: Option<String>,
}
