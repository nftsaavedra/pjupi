use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn current_timestamp_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

// ── Patentes ──────────────────────────────────────────────────────────────────

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

// ── Productos ─────────────────────────────────────────────────────────────────

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

// ── Equipamientos ─────────────────────────────────────────────────────────────

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

// ── Financiamientos ───────────────────────────────────────────────────────────

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
