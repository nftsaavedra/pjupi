#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Producto I+D+i derivado de investigación (software, prototipo, metodología, norma, etc.).
/// La asociación a proyecto es opcional (0..N por proyecto).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Producto {
    pub id_producto: String,
    /// Vínculo opcional a un proyecto institucional.
    #[serde(default)]
    pub proyecto_id: Option<String>,
    #[serde(default)]
    pub docente_id: Option<String>,
    pub nombre: String,
    /// Ej.: "software", "prototipo", "metodologia", "norma", "base_datos"
    #[serde(default)]
    pub tipo: Option<String>,
    #[serde(default)]
    pub descripcion: Option<String>,
    #[serde(default)]
    pub fecha_registro: Option<i64>,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl Producto {
    pub fn new(nombre: String, now_ms: i64) -> Self {
        Self {
            id_producto: Uuid::new_v4().to_string(),
            proyecto_id: None,
            docente_id: None,
            nombre,
            tipo: None,
            descripcion: None,
            fecha_registro: Some(now_ms),
            created_at: Some(now_ms),
            updated_at: Some(now_ms),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateProductoRequest {
    pub proyecto_id: Option<String>,
    pub docente_id: Option<String>,
    pub nombre: String,
    pub tipo: Option<String>,
    pub descripcion: Option<String>,
    pub fecha_registro: Option<i64>,
}
