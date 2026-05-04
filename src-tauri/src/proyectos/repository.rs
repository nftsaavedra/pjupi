use std::collections::{HashMap, HashSet};

use futures_util::TryStreamExt;
use mongodb::{bson::doc, Database};
use crate::shared::error::AppError;
use crate::proyectos::models::{
    Proyecto,
    CreateProyectoRequest,
    CreateProyectoConParticipantesRequest,
    UpdateProyectoConParticipantesRequest,
    ProyectoDetalle,
    ProyectoParticipanteResumen,
    EliminarProyectoResultado,
    DocenteProyectosCount,
    KpisDashboard,
    ExportData,
    ExportDataConProjectos,
};
use crate::proyectos::service;
use crate::docentes::models::Docente;
use crate::docentes::repository as docentes_repo;
use crate::grados::models::GradoAcademico;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct ParticipacionRecord {
    #[serde(rename = "_id")]
    id: String,
    id_proyecto: String,
    id_docente: String,
    #[serde(default)]
    es_responsable: bool,
}

async fn validate_docentes_activos(db: &Database, docentes_ids: &[String]) -> Result<(), AppError> {
    let docentes_activos = db.collection::<Docente>("docentes")
        .find(doc! { "id_docente": { "$in": docentes_ids }, "activo": 1i64 })
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    if docentes_activos.len() != docentes_ids.len() {
        return Err(AppError::InternalError(
            "Uno o más docentes seleccionados no existen o están inactivos.".to_string(),
        ));
    }

    Ok(())
}

async fn load_grados_map(db: &Database) -> Result<HashMap<String, GradoAcademico>, AppError> {
    let grados = db.collection::<GradoAcademico>("grados")
        .find(doc! {})
        .await?
        .try_collect::<Vec<_>>()
        .await?;
    Ok(grados.into_iter().map(|grado| (grado.id_grado.clone(), grado)).collect())
}

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

pub async fn create_proyecto_con_participantes(db: &Database, request: CreateProyectoConParticipantesRequest) -> Result<Proyecto, AppError> {
    let prepared = service::prepare_create_input(request)?;
    validate_docentes_activos(db, &prepared.docentes_ids).await?;

    let proyecto = Proyecto::new(CreateProyectoRequest { titulo_proyecto: prepared.titulo_proyecto });
    db.collection::<Proyecto>("proyectos").insert_one(&proyecto).await?;

    let participaciones_collection = db.collection::<ParticipacionRecord>("participaciones");
    for docente_id in prepared.docentes_ids {
        participaciones_collection.insert_one(ParticipacionRecord {
            id: format!("{}:{}", proyecto.id_proyecto, docente_id),
            id_proyecto: proyecto.id_proyecto.clone(),
            es_responsable: prepared.docente_responsable_id.as_deref() == Some(docente_id.as_str()),
            id_docente: docente_id,
        }).await?;
    }

    Ok(proyecto)
}

pub async fn update_proyecto_con_participantes(
    db: &Database,
    id_proyecto: &str,
    request: UpdateProyectoConParticipantesRequest,
) -> Result<Proyecto, AppError> {
    let prepared = service::prepare_update_input(request)?;
    validate_docentes_activos(db, &prepared.docentes_ids).await?;

    let proyecto_exists = db.collection::<Proyecto>("proyectos")
        .find_one(doc! { "id_proyecto": id_proyecto })
        .await?;

    if proyecto_exists.is_none() {
        return Err(AppError::NotFound("Proyecto no encontrado.".to_string()));
    }

    db.collection::<mongodb::bson::Document>("proyectos")
        .update_one(
            doc! { "id_proyecto": id_proyecto },
            doc! { "$set": { "titulo_proyecto": &prepared.titulo_proyecto } },
        )
        .await?;

    db.collection::<mongodb::bson::Document>("participaciones")
        .delete_many(doc! { "id_proyecto": id_proyecto })
        .await?;

    let participaciones_collection = db.collection::<ParticipacionRecord>("participaciones");
    for docente_id in prepared.docentes_ids {
        participaciones_collection.insert_one(ParticipacionRecord {
            id: format!("{}:{}", id_proyecto, docente_id),
            id_proyecto: id_proyecto.to_string(),
            es_responsable: prepared.docente_responsable_id.as_deref() == Some(docente_id.as_str()),
            id_docente: docente_id,
        }).await?;
    }

    db.collection::<Proyecto>("proyectos")
        .find_one(doc! { "id_proyecto": id_proyecto })
        .await?
        .ok_or_else(|| AppError::NotFound("Proyecto no encontrado.".to_string()))
}

pub async fn buscar_proyectos_por_docente(db: &Database, id_docente: &str) -> Result<Vec<Proyecto>, AppError> {
    let participaciones = db.collection::<ParticipacionRecord>("participaciones")
        .find(doc! { "id_docente": id_docente })
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let proyecto_ids: Vec<String> = participaciones.into_iter().map(|item| item.id_proyecto).collect();
    if proyecto_ids.is_empty() {
        return Ok(Vec::new());
    }

    let mut proyectos = db.collection::<Proyecto>("proyectos")
        .find(doc! { "id_proyecto": { "$in": proyecto_ids }, "activo": 1i64 })
        .await?
        .try_collect::<Vec<_>>()
        .await?;
    proyectos.sort_by(|a, b| a.titulo_proyecto.to_lowercase().cmp(&b.titulo_proyecto.to_lowercase()));
    Ok(proyectos)
}

pub async fn get_all_proyectos_detalle(db: &Database) -> Result<Vec<ProyectoDetalle>, AppError> {
    let mut proyectos = db.collection::<Proyecto>("proyectos")
        .find(doc! {})
        .await?
        .try_collect::<Vec<_>>()
        .await?;
    proyectos.sort_by(|a, b| a.titulo_proyecto.to_lowercase().cmp(&b.titulo_proyecto.to_lowercase()));

    let docentes = load_docentes_map(db).await?;
    let grados = load_grados_map(db).await?;
    let participaciones = load_participaciones(db).await?;

    let mut docentes_por_proyecto: HashMap<String, Vec<String>> = HashMap::new();
    let mut participantes_por_proyecto: HashMap<String, Vec<ProyectoParticipanteResumen>> = HashMap::new();
    for participacion in participaciones {
        if let Some(docente) = docentes.get(&participacion.id_docente) {
            let proyecto_id = participacion.id_proyecto.clone();
            let grado = grados
                .get(&docente.id_grado)
                .map(|item| item.nombre.clone())
                .unwrap_or_else(|| "Sin grado".to_string());
            let nivel_renacyt = docente
                .renacyt_nivel
                .clone()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "No registrado".to_string());
            docentes_por_proyecto
                .entry(proyecto_id.clone())
                .or_default()
                .push(format!("{} ({} · {})", docente.nombres_apellidos, grado, nivel_renacyt));
            participantes_por_proyecto
                .entry(proyecto_id)
                .or_default()
                .push(ProyectoParticipanteResumen {
                    id_docente: docente.id_docente.clone(),
                    nombre: docente.nombres_apellidos.clone(),
                    grado,
                    renacyt_nivel: nivel_renacyt,
                    es_responsable: participacion.es_responsable,
                });
        }
    }

    let detalles = proyectos
        .into_iter()
        .map(|proyecto| {
            let proyecto_id = proyecto.id_proyecto.clone();
            let docentes_proyecto = docentes_por_proyecto.remove(&proyecto_id).unwrap_or_default();
            let mut participantes = participantes_por_proyecto.remove(&proyecto_id).unwrap_or_default();
            participantes.sort_by(|a, b| b.es_responsable.cmp(&a.es_responsable).then_with(|| a.nombre.to_lowercase().cmp(&b.nombre.to_lowercase())));
            let docente_responsable = participantes
                .iter()
                .find(|participante| participante.es_responsable)
                .map(|participante| participante.nombre.clone());
            ProyectoDetalle {
                id_proyecto: proyecto.id_proyecto,
                titulo_proyecto: proyecto.titulo_proyecto,
                cantidad_docentes: docentes_proyecto.len() as i64,
                docente_responsable,
                docentes: if docentes_proyecto.is_empty() {
                    None
                } else {
                    Some(docentes_proyecto.join(" | "))
                },
                participantes_json: if docentes_proyecto.is_empty() {
                    None
                } else {
                    serde_json::to_string(&participantes).ok()
                },
                activo: proyecto.activo,
            }
        })
        .collect();

    Ok(detalles)
}

pub async fn eliminar_relacion_proyecto_docente(db: &Database, id_proyecto: &str, id_docente: &str) -> Result<(), AppError> {
    let relation_id = format!("{}:{}", id_proyecto, id_docente);
    db.collection::<mongodb::bson::Document>("participaciones")
        .delete_one(doc! { "_id": relation_id })
        .await?;
    Ok(())
}

pub async fn eliminar_relaciones_proyecto(db: &Database, id_proyecto: &str) -> Result<(), AppError> {
    db.collection::<mongodb::bson::Document>("participaciones")
        .delete_many(doc! { "id_proyecto": id_proyecto })
        .await?;
    Ok(())
}

pub async fn eliminar_proyecto(db: &Database, id_proyecto: &str) -> Result<EliminarProyectoResultado, AppError> {
    let docentes_relacionados = db.collection::<mongodb::bson::Document>("participaciones")
        .count_documents(doc! { "id_proyecto": id_proyecto })
        .await?;

    if docentes_relacionados > 0 {
        return Err(AppError::InternalError(
            "No se puede eliminar el proyecto porque aún tiene docentes relacionados. Elimine primero esas relaciones.".to_string(),
        ));
    }

    db.collection::<mongodb::bson::Document>("proyectos")
        .update_one(doc! { "id_proyecto": id_proyecto }, doc! { "$set": { "activo": 0i64 } })
        .await?;

    Ok(EliminarProyectoResultado {
        accion: "desactivado".to_string(),
        mensaje: "Proyecto desactivado correctamente.".to_string(),
    })
}

pub async fn reactivar_proyecto(db: &Database, id_proyecto: &str) -> Result<Proyecto, AppError> {
    db.collection::<mongodb::bson::Document>("proyectos")
        .update_one(doc! { "id_proyecto": id_proyecto }, doc! { "$set": { "activo": 1i64 } })
        .await?;

    db.collection::<Proyecto>("proyectos")
        .find_one(doc! { "id_proyecto": id_proyecto })
        .await?
        .ok_or_else(|| AppError::NotFound("Proyecto no encontrado.".to_string()))
}

pub async fn get_estadisticas_proyectos_x_docente(db: &Database) -> Result<Vec<DocenteProyectosCount>, AppError> {
    let docentes = docentes_repo::get_all_docentes(db).await?;
    let proyectos = load_proyectos_map(db).await?;
    let participaciones = load_participaciones(db).await?;

    let mut activos_por_docente: HashMap<String, i64> = docentes
        .iter()
        .map(|docente| (docente.id_docente.clone(), 0))
        .collect();

    for participacion in participaciones {
        if let Some(proyecto) = proyectos.get(&participacion.id_proyecto) {
            if proyecto.activo == 1 {
                if let Some(contador) = activos_por_docente.get_mut(&participacion.id_docente) {
                    *contador += 1;
                }
            }
        }
    }

    let mut stats: Vec<DocenteProyectosCount> = docentes
        .into_iter()
        .map(|docente| DocenteProyectosCount {
            nombre: docente.nombres_apellidos,
            cantidad: *activos_por_docente.get(&docente.id_docente).unwrap_or(&0),
        })
        .collect();
    stats.sort_by(|a, b| b.cantidad.cmp(&a.cantidad).then_with(|| a.nombre.cmp(&b.nombre)));
    Ok(stats)
}

pub async fn get_kpis_dashboard(db: &Database) -> Result<KpisDashboard, AppError> {
    let docentes = docentes_repo::get_all_docentes(db).await?;
    let proyectos = db.collection::<mongodb::bson::Document>("proyectos")
        .count_documents(doc! { "activo": 1i64 })
        .await? as i64;
    let stats = get_estadisticas_proyectos_x_docente(db).await?;

    let docentes_con_1_proyecto = stats.iter().filter(|item| item.cantidad == 1).count() as i64;
    let docentes_multiples_proyectos = stats.iter().filter(|item| item.cantidad > 1).count() as i64;

    Ok(KpisDashboard {
        total_proyectos: proyectos,
        total_docentes: docentes.len() as i64,
        docentes_con_1_proyecto,
        docentes_multiples_proyectos,
    })
}

pub async fn get_data_exportacion_plana(db: &Database) -> Result<Vec<ExportData>, AppError> {
    let grados = load_grados_map(db).await?;
    let docentes = load_docentes_map(db).await?;
    let proyectos = load_proyectos_map(db).await?;
    let participaciones = load_participaciones(db).await?;

    let mut data = Vec::new();
    for participacion in participaciones {
        let Some(proyecto) = proyectos.get(&participacion.id_proyecto) else {
            continue;
        };
        let Some(docente) = docentes.get(&participacion.id_docente) else {
            continue;
        };
        if proyecto.activo != 1 || docente.activo != 1 {
            continue;
        }
        let grado = grados
            .get(&docente.id_grado)
            .map(|item| item.nombre.clone())
            .unwrap_or_else(|| "Sin grado".to_string());

        data.push(ExportData {
            proyecto: proyecto.titulo_proyecto.clone(),
            grado,
            renacyt_nivel: docente
                .renacyt_nivel
                .clone()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "No registrado".to_string()),
            docente: docente.nombres_apellidos.clone(),
            dni: docente.dni.clone(),
        });
    }

    data.sort_by(|a, b| a.proyecto.cmp(&b.proyecto).then_with(|| a.docente.cmp(&b.docente)));
    Ok(data)
}

pub async fn get_data_exportacion_agrupada_docente(db: &Database) -> Result<Vec<ExportDataConProjectos>, AppError> {
    let grados = load_grados_map(db).await?;
    let docentes_activos = docentes_repo::get_all_docentes(db).await?;
    let proyectos = load_proyectos_map(db).await?;
    let participaciones = load_participaciones(db).await?;

    let docentes_ids: HashSet<String> = docentes_activos.iter().map(|docente| docente.id_docente.clone()).collect();
    let mut proyectos_por_docente: HashMap<String, Vec<String>> = HashMap::new();

    for participacion in participaciones {
        if !docentes_ids.contains(&participacion.id_docente) {
            continue;
        }
        if let Some(proyecto) = proyectos.get(&participacion.id_proyecto) {
            if proyecto.activo == 1 {
                proyectos_por_docente
                    .entry(participacion.id_docente)
                    .or_default()
                    .push(proyecto.titulo_proyecto.clone());
            }
        }
    }

    let mut data: Vec<ExportDataConProjectos> = docentes_activos
        .into_iter()
        .map(|docente| {
            let proyectos_docente = proyectos_por_docente.remove(&docente.id_docente).unwrap_or_default();
            ExportDataConProjectos {
                docente: docente.nombres_apellidos,
                dni: docente.dni,
                grado: grados
                    .get(&docente.id_grado)
                    .map(|grado| grado.nombre.clone())
                    .unwrap_or_else(|| "Sin grado".to_string()),
                renacyt_nivel: docente
                    .renacyt_nivel
                    .filter(|value| !value.trim().is_empty())
                    .unwrap_or_else(|| "No registrado".to_string()),
                cantidad_proyectos: proyectos_docente.len() as i64,
                proyectos: if proyectos_docente.is_empty() {
                    None
                } else {
                    Some(proyectos_docente.join(" | "))
                },
            }
        })
        .collect();

    data.sort_by(|a, b| a.docente.cmp(&b.docente));
    Ok(data)
}
