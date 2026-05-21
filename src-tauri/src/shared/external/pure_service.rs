use futures_util::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::UpdateOptions;

use crate::docentes::models::{Docente, Publicacion, SyncPublicacionesResult};
use crate::shared::error::AppError;
use crate::shared::external::pure_client;
use crate::shared::state::AppState;
use crate::shared::time;

pub async fn sync_publicaciones(
    state: &AppState,
    docente_id: &str,
) -> Result<SyncPublicacionesResult, AppError> {
    let db = state.mongo_db()?;

    let docente = db
        .collection::<Docente>("docentes")
        .find_one(doc! { "id_docente": docente_id })
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

    let pure_person_uuid = pure_client::resolve_person_uuid(&state.pure_config, scopus_author_id)
        .await
        .unwrap_or(None);

    let fetched =
        pure_client::fetch_research_outputs_by_scopus_id(&state.pure_config, scopus_author_id)
            .await?;

    let total_encontradas = fetched.len();
    let mut nuevas = 0usize;
    let mut actualizadas = 0usize;
    let now_ms = time::now_ms();

    let col = db.collection::<Publicacion>("publicaciones");

    for fp in fetched {
        let filter = doc! { "pure_uuid": &fp.pure_uuid };

        let set_doc = doc! {
            "docente_id":           docente_id,
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
        docente_id: docente_id.to_string(),
        scopus_author_id: scopus_author_id.to_string(),
        pure_person_uuid,
        total_encontradas,
        nuevas,
        actualizadas,
    })
}

pub async fn get_publicaciones(
    state: &AppState,
    docente_id: &str,
) -> Result<Vec<Publicacion>, AppError> {
    let db = state.mongo_db()?;
    let publicaciones = db
        .collection::<Publicacion>("publicaciones")
        .find(doc! { "docente_id": docente_id })
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    Ok(publicaciones)
}
