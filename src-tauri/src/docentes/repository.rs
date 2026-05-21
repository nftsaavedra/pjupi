use std::collections::HashMap;

use futures_util::TryStreamExt;
use mongodb::{bson::doc, Database};
use crate::shared::error::AppError;
use crate::shared::pagination::PaginatedResult;
use crate::docentes::models::{CreateDocenteRequest, Docente, DocenteDetalle, EliminarDocenteResultado};
use crate::docentes::service::build_delete_result;

use crate::shared::data_loader;

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

pub async fn get_all_docentes_paginated(db: &Database, page: u32, limit: u32) -> Result<PaginatedResult<Docente>, AppError> {
    let skip = (page.saturating_sub(1) * limit) as u64;
    let limit_i64 = limit as i64;
    let filter = doc! { "activo": 1i64 };

    let total = db.collection::<Docente>("docentes")
        .count_documents(filter.clone())
        .await?;

    let mut cursor = db.collection::<Docente>("docentes")
        .find(filter)
        .sort(doc! { "nombres_apellidos": 1 })
        .skip(skip)
        .limit(limit_i64)
        .await?;

    let mut docentes: Vec<Docente> = Vec::new();
    while let Some(docente) = cursor.try_next().await? {
        docentes.push(docente);
    }

    let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
    Ok(PaginatedResult { items: docentes, total, page, limit, total_pages })
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

    let grados = data_loader::load_grados_map(db).await?;
    let proyectos = data_loader::load_proyectos_map(db).await?;
    let participaciones = data_loader::load_participaciones(db).await?;

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

            DocenteDetalle::from((docente, grado, proyectos_docente))
        })
        .collect();

    Ok(detalles)
}

pub async fn get_docente_detalle_by_id(db: &Database, id_docente: &str) -> Result<DocenteDetalle, AppError> {
    let docente = get_docente_by_id(db, id_docente).await?;
    let grados = data_loader::load_grados_map(db).await?;
    let proyectos = data_loader::load_proyectos_map(db).await?;
    let participaciones = data_loader::load_participaciones(db).await?;

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

    Ok(DocenteDetalle::from((docente, grado, proyectos_docente)))
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
