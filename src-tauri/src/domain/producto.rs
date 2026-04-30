use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn current_timestamp_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Producto {
    #[serde(rename = "_id")]
    pub id: String,
    pub id_producto: String,
    pub proyecto_id: Option<String>,
    pub docente_id: Option<String>,
    pub nombre: String,
    /// "software" | "prototipo" | "metodologia" | "norma" | "base_datos"
    pub tipo: Option<String>,
    pub etapa: Option<String>,
    pub descripcion: Option<String>,
    pub fecha_registro: Option<i64>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

impl Producto {
    pub fn new(request: CreateProductoRequest) -> Self {
        let now = current_timestamp_ms();
        let id = Uuid::new_v4().to_string();
        Self {
            id: id.clone(),
            id_producto: id,
            proyecto_id: request.proyecto_id,
            docente_id: request.docente_id,
            nombre: request.nombre,
            tipo: request.tipo,
            etapa: request.etapa,
            descripcion: request.descripcion,
            fecha_registro: request.fecha_registro,
            created_at: Some(now),
            updated_at: Some(now),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateProductoRequest {
    pub proyecto_id: Option<String>,
    pub docente_id: Option<String>,
    pub nombre: String,
    pub tipo: Option<String>,
    pub etapa: Option<String>,
    pub descripcion: Option<String>,
    pub fecha_registro: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateProductoRequest {
    pub nombre: Option<String>,
    pub tipo: Option<String>,
    pub etapa: Option<String>,
    pub descripcion: Option<String>,
    pub fecha_registro: Option<i64>,
}
