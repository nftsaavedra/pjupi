use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CatalogoItem {
    pub id_catalogo: String,
    pub tipo: String,
    pub codigo: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub orden: Option<i32>,
    pub activo: i64,
    #[serde(default)]
    pub updated_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EliminarCatalogoResultado {
    pub accion: String,
    pub mensaje: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCatalogoRequest {
    pub tipo: String,
    pub codigo: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub orden: Option<i32>,
}

impl CatalogoItem {
    pub fn new(request: CreateCatalogoRequest) -> Self {
        let now = crate::shared::time::now_ms();
        Self {
            id_catalogo: Uuid::new_v4().to_string(),
            tipo: request.tipo,
            codigo: request.codigo,
            nombre: request.nombre,
            descripcion: request.descripcion,
            orden: request.orden,
            activo: 1,
            updated_at: Some(now),
        }
    }
}
