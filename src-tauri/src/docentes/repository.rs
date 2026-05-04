use std::collections::HashMap;

use futures_util::TryStreamExt;
use mongodb::{bson::doc, Database};
use crate::shared::error::AppError;
use crate::docentes::models::{CreateDocenteRequest, Docente, DocenteDetalle, EliminarDocenteResultado};
use crate::docentes::service::build_delete_result;
use crate::grados::models::GradoAcademico;
use crate::proyectos::models::Proyecto;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct ParticipacionRecord {
    #[serde(rename = "_id")]
    id: String,
    id_proyecto: String,
    id_docente: String,
    #[serde(default)]
    es_responsable: bool,
}

async fn load_grados_map(db: &Database) -> Result<HashMap<String, GradoAcademico>, AppError> {
    let grados = db.collection::<GradoAcademico>("grados")
        .find(doc! {})
        .await?
        .try_collect::<Vec<_>>()
        .await?;
    Ok(grados.into_iter().map(|grado| (grado.id_grado.clone(), grado)).collect())
}

#[allow(dead_code)]
async fn load_docentes_map(db: &Database) -> Result<HashMap<String, Docente>, AppError> {
    let docentes = db.collection::<Docente>("docentes")
        .find(doc! {})
        .await?
        .try_collect::<Vec<_>>()
        .await?;
    Ok(docentes.into_iter().map(|docente| (docente.id_docente.clone(), docente)).collect())
}

async fn load_proyectos_map(db: &Database) -> Result<HashMap<String, Proyecto>, AppError> {
    let proyectos = db.collection::<Proyecto>("proyectos")
        .find(doc! {})
        .await?
        .try_collect::<Vec<_>>()
        .await?;
    Ok(proyectos.into_iter().map(|proyecto| (proyecto.id_proyecto.clone(), proyecto)).collect())
}

async fn load_participaciones(db: &Database) -> Result<Vec<ParticipacionRecord>, AppError> {
    db.collection::<ParticipacionRecord>("participaciones")
        .find(doc! {})
        .await?
        .try_collect::<Vec<_>>()
        .await
        .map_err(Into::into)
}

pub async fn create_docente(db: &Database, request: CreateDocenteRequest) -> Result<Docente, AppError> {
    let grado_existente = db.collection::<mongodb::bson::Document>("grados")
        .find_one(doc! { "id_grado": &request.id_grado })
        .await?;
    if grado_existente.is_none() {
        return Err(AppError::NotFound("El grado seleccionado no existe.".to_string()));
    }

    let docente = Docente::new(request);
    db.collection::<Docente>("docentes").insert_one(&docente).await?;
    Ok(docente)
}

pub async fn get_all_docentes(db: &Database) -> Result<Vec<Docente>, AppError> {
    let mut docentes = db.collection::<Docente>("docentes")
        .find(doc! { "activo": 1i64 })
        .await?
        .try_collect::<Vec<_>>()
        .await?;
    docentes.sort_by(|a, b| a.nombres_apellidos.to_lowercase().cmp(&b.nombres_apellidos.to_lowercase()));
    Ok(docentes)
}

pub async fn get_docente_by_dni(db: &Database, dni: &str) -> Result<Option<Docente>, AppError> {
    db.collection::<Docente>("docentes")
        .find_one(doc! { "dni": dni })
        .await
        .map_err(Into::into)
}

pub async fn get_docente_by_id(db: &Database, id_docente: &str) -> Result<Docente, AppError> {
    db.collection::<Docente>("docentes")
        .find_one(doc! { "id_docente": id_docente })
        .await?
        .ok_or_else(|| AppError::NotFound("Docente no encontrado.".to_string()))
}

pub async fn update_docente_renacyt(db: &Database, docente: &Docente) -> Result<(), AppError> {
    db.collection::<Docente>("docentes")
        .replace_one(doc! { "id_docente": &docente.id_docente }, docente)
        .await?;

    Ok(())
}

pub async fn get_all_docentes_con_proyectos(db: &Database) -> Result<Vec<DocenteDetalle>, AppError> {
    let mut docentes = db.collection::<Docente>("docentes")
        .find(doc! {})
        .await?
        .try_collect::<Vec<_>>()
        .await?;
    docentes.sort_by(|a, b| a.nombres_apellidos.to_lowercase().cmp(&b.nombres_apellidos.to_lowercase()));

    let grados = load_grados_map(db).await?;
    let proyectos = load_proyectos_map(db).await?;
    let participaciones = load_participaciones(db).await?;

    let mut proyectos_por_docente: HashMap<String, Vec<String>> = HashMap::new();
    for participacion in participaciones {
        if let Some(proyecto) = proyectos.get(&participacion.id_proyecto) {
            if proyecto.activo == 1 {
                proyectos_por_docente
                    .entry(participacion.id_docente)
                    .or_default()
                    .push(proyecto.titulo_proyecto.clone());
            }
        }
    }

    let detalles = docentes
        .into_iter()
        .map(|docente| {
            let proyectos_docente = proyectos_por_docente.remove(&docente.id_docente).unwrap_or_default();
            let grado = grados
                .get(&docente.id_grado)
                .map(|grado| grado.nombre.clone())
                .unwrap_or_else(|| "Sin grado".to_string());

            DocenteDetalle {
                id_docente: docente.id_docente,
                dni: docente.dni,
                nombres_apellidos: docente.nombres_apellidos,
                nombres: docente.nombres,
                apellido_paterno: docente.apellido_paterno,
                apellido_materno: docente.apellido_materno,
                grado,
                cantidad_proyectos: proyectos_docente.len() as i64,
                proyectos: if proyectos_docente.is_empty() {
                    None
                } else {
                    Some(proyectos_docente.join(" | "))
                },
                activo: docente.activo,
                renacyt_codigo_registro: docente.renacyt_codigo_registro,
                renacyt_id_investigador: docente.renacyt_id_investigador,
                renacyt_nivel: docente.renacyt_nivel,
                renacyt_grupo: docente.renacyt_grupo,
                renacyt_condicion: docente.renacyt_condicion,
                renacyt_fecha_informe_calificacion: docente.renacyt_fecha_informe_calificacion,
                renacyt_fecha_registro: docente.renacyt_fecha_registro,
                renacyt_fecha_ultima_revision: docente.renacyt_fecha_ultima_revision,
                renacyt_orcid: docente.renacyt_orcid,
                renacyt_scopus_author_id: docente.renacyt_scopus_author_id,
                renacyt_fecha_ultima_sincronizacion: docente.renacyt_fecha_ultima_sincronizacion,
                renacyt_ficha_url: docente.renacyt_ficha_url,
                renacyt_formaciones_academicas_json: docente.renacyt_formaciones_academicas_json,
            }
        })
        .collect();

    Ok(detalles)
}

pub async fn get_docente_detalle_by_id(db: &Database, id_docente: &str) -> Result<DocenteDetalle, AppError> {
    let docente = get_docente_by_id(db, id_docente).await?;
    let grados = load_grados_map(db).await?;
    let proyectos = load_proyectos_map(db).await?;
    let participaciones = load_participaciones(db).await?;

    let proyectos_docente = participaciones
        .into_iter()
        .filter(|participacion| participacion.id_docente == docente.id_docente)
        .filter_map(|participacion| proyectos.get(&participacion.id_proyecto))
        .filter(|proyecto| proyecto.activo == 1)
        .map(|proyecto| proyecto.titulo_proyecto.clone())
        .collect::<Vec<_>>();

    let grado = grados
        .get(&docente.id_grado)
        .map(|grado| grado.nombre.clone())
        .unwrap_or_else(|| "Sin grado".to_string());

    Ok(DocenteDetalle {
        id_docente: docente.id_docente,
        dni: docente.dni,
        nombres_apellidos: docente.nombres_apellidos,
        nombres: docente.nombres,
        apellido_paterno: docente.apellido_paterno,
        apellido_materno: docente.apellido_materno,
        grado,
        cantidad_proyectos: proyectos_docente.len() as i64,
        proyectos: if proyectos_docente.is_empty() {
            None
        } else {
            Some(proyectos_docente.join(" | "))
        },
        activo: docente.activo,
        renacyt_codigo_registro: docente.renacyt_codigo_registro,
        renacyt_id_investigador: docente.renacyt_id_investigador,
        renacyt_nivel: docente.renacyt_nivel,
        renacyt_grupo: docente.renacyt_grupo,
        renacyt_condicion: docente.renacyt_condicion,
        renacyt_fecha_informe_calificacion: docente.renacyt_fecha_informe_calificacion,
        renacyt_fecha_registro: docente.renacyt_fecha_registro,
        renacyt_fecha_ultima_revision: docente.renacyt_fecha_ultima_revision,
        renacyt_orcid: docente.renacyt_orcid,
        renacyt_scopus_author_id: docente.renacyt_scopus_author_id,
        renacyt_fecha_ultima_sincronizacion: docente.renacyt_fecha_ultima_sincronizacion,
        renacyt_ficha_url: docente.renacyt_ficha_url,
        renacyt_formaciones_academicas_json: docente.renacyt_formaciones_academicas_json,
    })
}

pub async fn delete_docente(db: &Database, id_docente: &str) -> Result<EliminarDocenteResultado, AppError> {
    let participaciones = db.collection::<mongodb::bson::Document>("participaciones")
        .count_documents(doc! { "id_docente": id_docente })
        .await?;

    db.collection::<mongodb::bson::Document>("docentes")
        .update_one(doc! { "id_docente": id_docente }, doc! { "$set": { "activo": 0i64 } })
        .await?;

    Ok(build_delete_result(participaciones > 0))
}

pub async fn reactivar_docente(db: &Database, id_docente: &str) -> Result<Docente, AppError> {
    db.collection::<mongodb::bson::Document>("docentes")
        .update_one(doc! { "id_docente": id_docente }, doc! { "$set": { "activo": 1i64 } })
        .await?;

    db.collection::<Docente>("docentes")
        .find_one(doc! { "id_docente": id_docente })
        .await?
        .ok_or_else(|| AppError::NotFound("Docente no encontrado.".to_string()))
}
