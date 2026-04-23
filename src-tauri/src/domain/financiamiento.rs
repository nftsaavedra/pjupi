use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Financiamiento externo o interno asociado a un proyecto de investigación.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Financiamiento {
    pub id_financiamiento: String,
    #[serde(default)]
    pub proyecto_id: Option<String>,
    pub entidad_financiadora: String,
    /// Ej.: "nacional", "internacional", "propio", "concursable"
    #[serde(default)]
    pub tipo: Option<String>,
    #[serde(default)]
    pub monto: Option<f64>,
    #[serde(default)]
    pub moneda: Option<String>,
    #[serde(default)]
    pub fecha_inicio: Option<i64>,
    #[serde(default)]
    pub fecha_fin: Option<i64>,
    #[serde(default)]
    pub descripcion: Option<String>,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl Financiamiento {
    pub fn new(entidad_financiadora: String, now_ms: i64) -> Self {
        Self {
            id_financiamiento: Uuid::new_v4().to_string(),
            proyecto_id: None,
            entidad_financiadora,
            tipo: None,
            monto: None,
            moneda: None,
            fecha_inicio: None,
            fecha_fin: None,
            descripcion: None,
            created_at: Some(now_ms),
            updated_at: Some(now_ms),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateFinanciamientoRequest {
    pub proyecto_id: Option<String>,
    pub entidad_financiadora: String,
    pub tipo: Option<String>,
    pub monto: Option<f64>,
    pub moneda: Option<String>,
    pub fecha_inicio: Option<i64>,
    pub fecha_fin: Option<i64>,
    pub descripcion: Option<String>,
}
