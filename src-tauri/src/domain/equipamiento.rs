use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn current_timestamp_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Equipamiento {
    #[serde(rename = "_id")]
    pub id: String,
    pub id_equipamiento: String,
    pub proyecto_id: Option<String>,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub especificaciones: Option<String>,
    pub valor_estimado: Option<f64>,
    pub moneda: Option<String>,
    pub proveedor: Option<String>,
    pub fecha_adquisicion: Option<i64>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

impl Equipamiento {
    pub fn new(request: CreateEquipamientoRequest) -> Self {
        let now = current_timestamp_ms();
        let id = Uuid::new_v4().to_string();
        Self {
            id: id.clone(),
            id_equipamiento: id,
            proyecto_id: request.proyecto_id,
            nombre: request.nombre,
            descripcion: request.descripcion,
            especificaciones: request.especificaciones,
            valor_estimado: request.valor_estimado,
            moneda: request.moneda,
            proveedor: request.proveedor,
            fecha_adquisicion: request.fecha_adquisicion,
            created_at: Some(now),
            updated_at: Some(now),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateEquipamientoRequest {
    pub proyecto_id: Option<String>,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub especificaciones: Option<String>,
    pub valor_estimado: Option<f64>,
    pub moneda: Option<String>,
    pub proveedor: Option<String>,
    pub fecha_adquisicion: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateEquipamientoRequest {
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
    pub especificaciones: Option<String>,
    pub valor_estimado: Option<f64>,
    pub moneda: Option<String>,
    pub proveedor: Option<String>,
    pub fecha_adquisicion: Option<i64>,
}
