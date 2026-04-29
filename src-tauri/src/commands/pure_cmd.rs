use chrono::Utc;
use mongodb::bson::doc;
use mongodb::options::UpdateOptions;
use tauri::{State, Window};

use crate::domain::docente::Docente;
use crate::domain::publicacion::{Publicacion, SyncPublicacionesResult};
use crate::error::AppError;
use crate::infrastructure::pure_client;
use crate::state::AppState;
use crate::storage;

#[tauri::command]
pub async fn sincronizar_publicaciones_pure(
    window: Window,
    state: State<'_, AppState>,
    docente_id: String,
) -> Result<SyncPublicacionesResult, AppError> {
    // 1. Verificar permiso (operador o admin)
    storage::require_docentes_manage_permission(&state, window.label()).await?;

    // 2. Obtener base de datos MongoDB
    let db = state.mongo_db()?;

    // 3. Resolver docente y obtener Scopus Author ID canónico
    let docente = db
        .collection::<Docente>("docentes")
        .find_one(doc! { "id_docente": &docente_id })
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Docente '{}' no encontrado.", docente_id)))?;

    let scopus_author_id = docente
        .renacyt_scopus_author_id
        .as_deref()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            AppError::InternalError(
                "El docente no tiene un Scopus Author ID registrado. \
                Sincronice primero los datos RENACYT del docente para obtenerlo."
                    .to_string(),
            )
        })?;

    // 4. Intentar resolver UUID de persona en Pure (informativo; la búsqueda principal usa scopus_id)
    let pure_person_uuid = pure_client::resolve_person_uuid(&state.pure_config, scopus_author_id)
        .await
        .unwrap_or(None); // Si falla, continuamos con searchString directo

    // 5. Descargar todas las publicaciones asociadas al Scopus Author ID
    let fetched =
        pure_client::fetch_research_outputs_by_scopus_id(&state.pure_config, scopus_author_id)
            .await?;

    let total_encontradas = fetched.len();
    let mut nuevas = 0usize;
    let mut actualizadas = 0usize;
    let now_ms = Utc::now().timestamp_millis();

    let col = db.collection::<Publicacion>("publicaciones");

    // 6. Upsert idempotente por pure_uuid
    for fp in fetched {
        let filter = doc! { "pure_uuid": &fp.pure_uuid };

        // Construir $set: campos que siempre se actualizan
        let set_doc = doc! {
            "docente_id":           &docente_id,
            "titulo":               &fp.titulo,
            "tipo_publicacion":     fp.tipo_publicacion.as_deref(),
            "doi":                  fp.doi.as_deref(),
            "scopus_eid":           fp.scopus_eid.as_deref(),
            "anio_publicacion":     fp.anio_publicacion,
            "autores_json":         &fp.autores_json,
            "estado_publicacion":   fp.estado_publicacion.as_deref(),
            "journal_titulo":       fp.journal_titulo.as_deref(),
            "issn":                 fp.issn.as_deref(),
            "pure_sincronizado_at": now_ms,
            "updated_at":           now_ms,
        };

        // $setOnInsert: campos que solo se escriben al crear
        let new_id = uuid::Uuid::new_v4().to_string();
        let set_on_insert_doc = doc! {
            "id_publicacion": &new_id,
            "pure_uuid":      &fp.pure_uuid,
            "proyecto_id":    mongodb::bson::Bson::Null,
            "created_at":     now_ms,
        };

        let update = doc! {
            "$set":         set_doc,
            "$setOnInsert": set_on_insert_doc,
        };

        let opts = UpdateOptions::builder().upsert(true).build();
        let result = col.update_one(filter, update).with_options(opts).await?;

        if result.upserted_id.is_some() {
            nuevas += 1;
        } else if result.modified_count > 0 {
            actualizadas += 1;
        }
    }

    Ok(SyncPublicacionesResult {
        docente_id,
        scopus_author_id: scopus_author_id.to_string(),
        pure_person_uuid,
        total_encontradas,
        nuevas,
        actualizadas,
    })
}

#[tauri::command]
pub async fn get_publicaciones_docente(
    window: Window,
    state: State<'_, AppState>,
    docente_id: String,
) -> Result<Vec<Publicacion>, AppError> {
    use futures_util::TryStreamExt;

    storage::require_docentes_view_permission(&state, window.label()).await?;

    let db = state.mongo_db()?;
    let publicaciones = db
        .collection::<Publicacion>("publicaciones")
        .find(doc! { "docente_id": &docente_id })
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    Ok(publicaciones)
}
