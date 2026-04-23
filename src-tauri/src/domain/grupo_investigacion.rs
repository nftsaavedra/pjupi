use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Grupo de investigación institucional.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GrupoInvestigacion {
    pub id_grupo: String,
    pub nombre: String,
    #[serde(default)]
    pub descripcion: Option<String>,
    /// docente_id del docente que coordina el grupo.
    #[serde(default)]
    pub coordinador_id: Option<String>,
    /// Líneas temáticas del grupo (lista libre).
    #[serde(default)]
    pub lineas_investigacion: Vec<String>,
    pub activo: i64,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl GrupoInvestigacion {
    pub fn new(nombre: String, now_ms: i64) -> Self {
        Self {
            id_grupo: Uuid::new_v4().to_string(),
            nombre,
            descripcion: None,
            coordinador_id: None,
            lineas_investigacion: Vec::new(),
            activo: 1,
            created_at: Some(now_ms),
            updated_at: Some(now_ms),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateGrupoInvestigacionRequest {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub coordinador_id: Option<String>,
    #[serde(default)]
    pub lineas_investigacion: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGrupoInvestigacionRequest {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub coordinador_id: Option<String>,
    #[serde(default)]
    pub lineas_investigacion: Vec<String>,
}
