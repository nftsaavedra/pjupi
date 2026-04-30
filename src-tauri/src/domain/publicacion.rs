use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Publicación científica sincronizada desde Pure (Elsevier) por Scopus Author ID.
/// Puede estar asociada opcionalmente a un proyecto institucional.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Publicacion {
    pub id_publicacion: String,
    /// UUID único asignado por Pure. Usado como clave de upsert idempotente.
    pub pure_uuid: String,
    /// Docente al que pertenece esta publicación (obligatorio).
    pub docente_id: String,
    /// Vínculo opcional a un proyecto institucional (0..N por proyecto).
    #[serde(default)]
    pub proyecto_id: Option<String>,
    pub titulo: String,
    #[serde(default)]
    pub tipo_publicacion: Option<String>,
    #[serde(default)]
    pub doi: Option<String>,
    /// Identificador Scopus de la publicación (distinto del Scopus Author ID del docente).
    #[serde(default)]
    pub scopus_eid: Option<String>,
    #[serde(default)]
    pub anio_publicacion: Option<i32>,
    /// JSON array de nombres de contribuidores tal como los devuelve Pure.
    #[serde(default)]
    pub autores_json: Option<String>,
    #[serde(default)]
    pub estado_publicacion: Option<String>,
    #[serde(default)]
    pub journal_titulo: Option<String>,
    #[serde(default)]
    pub issn: Option<String>,
    /// Timestamp (ms epoch) de la última sincronización exitosa desde Pure.
    #[serde(default)]
    pub pure_sincronizado_at: Option<i64>,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl Publicacion {
    #[allow(dead_code)]
    pub fn new(
        pure_uuid: String,
        docente_id: String,
        titulo: String,
        now_ms: i64,
    ) -> Self {
        Self {
            id_publicacion: Uuid::new_v4().to_string(),
            pure_uuid,
            docente_id,
            proyecto_id: None,
            titulo,
            tipo_publicacion: None,
            doi: None,
            scopus_eid: None,
            anio_publicacion: None,
            autores_json: None,
            estado_publicacion: None,
            journal_titulo: None,
            issn: None,
            pure_sincronizado_at: Some(now_ms),
            created_at: Some(now_ms),
            updated_at: Some(now_ms),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncPublicacionesResult {
    pub docente_id: String,
    pub scopus_author_id: String,
    pub pure_person_uuid: Option<String>,
    pub total_encontradas: usize,
    pub nuevas: usize,
    pub actualizadas: usize,
}
