use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::time;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProyectoParticipanteResumen {
    pub id_docente: String,
    pub nombre: String,
    pub grado: String,
    pub renacyt_nivel: String,
    pub es_responsable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proyecto {
    pub id_proyecto: String,
    pub titulo_proyecto: String,
    pub activo: i64,
    #[serde(default)]
    pub updated_at: Option<i64>,
    /// Código OCDE del área temática del proyecto (ej. "1.1 Matemáticas").
    #[serde(default)]
    pub campo_ocde: Option<String>,
    /// Programas de investigación institucionales relacionados.
    #[serde(default)]
    pub programas_relacionados: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProyectoRequest {
    pub titulo_proyecto: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProyectoConParticipantesRequest {
    pub titulo_proyecto: String,
    pub docentes_ids: Vec<String>,
    pub docente_responsable_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProyectoConParticipantesRequest {
    pub titulo_proyecto: String,
    pub docentes_ids: Vec<String>,
    pub docente_responsable_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProyectoDetalle {
    pub id_proyecto: String,
    pub titulo_proyecto: String,
    pub cantidad_docentes: i64,
    pub docente_responsable: Option<String>,
    pub docentes: Option<String>,
    pub participantes_json: Option<String>,
    pub activo: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EliminarProyectoResultado {
    pub accion: String, // "desactivado"
    pub mensaje: String,
}

// NEW: Enhanced export data grouped by docente
#[derive(Debug, Serialize)]
pub struct ExportDataConProjectos {
    pub docente: String,
    pub dni: String,
    pub grado: String,
    pub renacyt_nivel: String,
    pub grupo_investigacion: Option<String>,
    pub cantidad_proyectos: i64,
    pub proyectos: Option<String>, // comma-separated project titles
}

impl Proyecto {
    pub fn new(request: CreateProyectoRequest) -> Self {
        let now = time::now_ms();
        
        Self {
            id_proyecto: Uuid::new_v4().to_string(),
            titulo_proyecto: request.titulo_proyecto,
            activo: 1,
            updated_at: Some(now),
            campo_ocde: None,
            programas_relacionados: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocenteProyectosCount {
    pub nombre: String,
    pub cantidad: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProyectosTrendItem {
    pub anio: i32,
    pub mes: u32,
    pub cantidad: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenacytDistribucionItem {
    pub nivel: String,
    pub cantidad_docentes: i64,
    pub con_proyectos: i64,
    pub sin_proyectos: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KpisDashboard {
    // Counts only active entities.
    pub total_proyectos: i64,
    pub total_docentes: i64,
    pub docentes_con_1_proyecto: i64,
    pub docentes_multiples_proyectos: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportData {
    pub proyecto: String,
    pub grado: String,
    pub renacyt_nivel: String,
    pub docente: String,
    pub dni: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportDataGrupo {
    pub grupo: String,
    pub descripcion: Option<String>,
    pub coordinador: Option<String>,
    pub cantidad_miembros: i64,
    pub miembros: Option<String>,
    pub lineas_investigacion: Vec<String>,
    pub cantidad_proyectos: i64,
    pub proyectos: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportDataRecurso {
    pub tipo_recurso: String,
    pub titulo_o_nombre: String,
    pub proyecto: Option<String>,
    pub docente: Option<String>,
    pub tipo: Option<String>,
    pub estado: Option<String>,
    pub moneda: Option<String>,
    pub monto: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportDataDocentePerfil {
    pub dni: String,
    pub nombres_apellidos: String,
    pub grado: String,
    pub renacyt_nivel: Option<String>,
    pub renacyt_grupo: Option<String>,
    pub renacyt_condicion: Option<String>,
    pub renacyt_orcid: Option<String>,
    pub grupo_investigacion: Option<String>,
    pub cantidad_proyectos: i64,
    pub cantidad_publicaciones: i64,
    pub proyectos: Option<String>,
    pub activo: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportDataProyectoArea {
    pub area: String,
    pub cantidad_proyectos: i64,
    pub proyectos: Option<String>,
    pub cantidad_docentes: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParticipacionRecord {
    #[serde(rename = "_id")]
    pub id: String,
    pub id_proyecto: String,
    pub id_docente: String,
    #[serde(default)]
    pub es_responsable: bool,
}
