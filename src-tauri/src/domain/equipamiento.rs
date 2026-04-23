use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Equipamiento científico o tecnológico asociado a un proyecto de investigación.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Equipamiento {
    pub id_equipamiento: String,
    #[serde(default)]
    pub proyecto_id: Option<String>,
    pub nombre: String,
    #[serde(default)]
    pub descripcion: Option<String>,
    #[serde(default)]
    pub valor_estimado: Option<f64>,
    #[serde(default)]
    pub moneda: Option<String>,
    #[serde(default)]
    pub proveedor: Option<String>,
    #[serde(default)]
    pub fecha_adquisicion: Option<i64>,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl Equipamiento {
    pub fn new(nombre: String, now_ms: i64) -> Self {
        Self {
            id_equipamiento: Uuid::new_v4().to_string(),
            proyecto_id: None,
            nombre,
            descripcion: None,
            valor_estimado: None,
            moneda: None,
            proveedor: None,
            fecha_adquisicion: None,
            created_at: Some(now_ms),
            updated_at: Some(now_ms),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateEquipamientoRequest {
    pub proyecto_id: Option<String>,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub valor_estimado: Option<f64>,
    pub moneda: Option<String>,
    pub proveedor: Option<String>,
    pub fecha_adquisicion: Option<i64>,
}
