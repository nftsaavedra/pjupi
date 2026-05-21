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
    ProyectosTrendItem,
    RenacytDistribucionItem,
    ExportData,
    ExportDataConProjectos,
    ExportDataGrupo,
    ExportDataRecurso,
    ExportDataDocentePerfil,
    ExportDataProyectoArea,
};
use crate::proyectos::service;
use crate::docentes::models::{Docente, Publicacion};
use crate::docentes::repository as docentes_repo;
use crate::proyectos::models::ParticipacionRecord;
use crate::shared::data_loader;
use crate::recursos::repository as recursos_repo;
use crate::recursos::models::{Patente, Producto, Equipamiento, Financiamiento};
use crate::grupos::models::GrupoInvestigacion;
use crate::catalogos::models::CatalogoItem;

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


pub async fn create_proyecto_con_participantes(db: &Database, request: CreateProyectoConParticipantesRequest) -> Result<Proyecto, AppError> {
    let prepared = service::prepare_create_input(request)?;
    validate_docentes_activos(db, &prepared.docentes_ids).await?;

    let mut session = db.client().start_session().await?;
    session.start_transaction().await?;

    let result = async {
        let proyecto = Proyecto::new(CreateProyectoRequest { titulo_proyecto: prepared.titulo_proyecto });
        db.collection::<Proyecto>("proyectos")
            .insert_one(&proyecto)
            .session(&mut session)
            .await?;

        let participaciones_collection = db.collection::<ParticipacionRecord>("participaciones");
        for docente_id in prepared.docentes_ids {
            participaciones_collection.insert_one(ParticipacionRecord {
                id: format!("{}:{}", proyecto.id_proyecto, docente_id),
                id_proyecto: proyecto.id_proyecto.clone(),
                es_responsable: prepared.docente_responsable_id.as_deref() == Some(docente_id.as_str()),
                id_docente: docente_id,
            }).session(&mut session).await?;
        }

        session.commit_transaction().await?;
        Ok(proyecto)
    }.await;

    match result {
        Ok(proyecto) => Ok(proyecto),
        Err(error) => {
            let _ = session.abort_transaction().await;
            Err(error)
        }
    }
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

    let docentes = data_loader::load_docentes_map(db).await?;
    let grados = data_loader::load_grados_map(db).await?;
    let participaciones = data_loader::load_participaciones(db).await?;

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

    recursos_repo::delete_patentes_by_proyecto(db, id_proyecto).await?;
    recursos_repo::delete_productos_by_proyecto(db, id_proyecto).await?;
    recursos_repo::delete_equipamientos_by_proyecto(db, id_proyecto).await?;
    recursos_repo::delete_financiamientos_by_proyecto(db, id_proyecto).await?;

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
    let proyectos = data_loader::load_proyectos_map(db).await?;
    let participaciones = data_loader::load_participaciones(db).await?;

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
    let grados = data_loader::load_grados_map(db).await?;
    let docentes = data_loader::load_docentes_map(db).await?;
    let proyectos = data_loader::load_proyectos_map(db).await?;
    let participaciones = data_loader::load_participaciones(db).await?;

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
    let grados = data_loader::load_grados_map(db).await?;
    let grupos = data_loader::load_grupos_map(db).await?;
    let docentes_activos = docentes_repo::get_all_docentes(db).await?;
    let proyectos = data_loader::load_proyectos_map(db).await?;
    let participaciones = data_loader::load_participaciones(db).await?;

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
                grupo_investigacion: docente
                    .grupo_investigacion_id
                    .as_ref()
                    .and_then(|gid| grupos.get(gid))
                    .map(|g| g.nombre.clone()),
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

pub async fn get_data_exportacion_grupos(db: &Database) -> Result<Vec<ExportDataGrupo>, AppError> {
    let docentes = data_loader::load_docentes_map(db).await?;
    let proyectos = data_loader::load_proyectos_map(db).await?;
    let participaciones = data_loader::load_participaciones(db).await?;

    let grupos = db.collection::<GrupoInvestigacion>("grupos_investigacion")
        .find(doc! { "activo": 1i64 })
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let mut proyectos_por_docente: HashMap<String, HashSet<String>> = HashMap::new();
    for p in &participaciones {
        proyectos_por_docente
            .entry(p.id_docente.clone())
            .or_default()
            .insert(p.id_proyecto.clone());
    }

    let mut data = Vec::new();
    for grupo in grupos {
        let miembros: Vec<&Docente> = docentes
            .values()
            .filter(|d| d.grupo_investigacion_id.as_deref() == Some(&grupo.id_grupo))
            .collect();

        let miembros_nombres: Vec<String> = miembros
            .iter()
            .map(|d| d.nombres_apellidos.clone())
            .collect();

        let coordinador = grupo.coordinador_id.as_ref().and_then(|cid| {
            docentes.get(cid).map(|d| d.nombres_apellidos.clone())
        });

        let mut all_proyecto_ids = HashSet::new();
        for m in &miembros {
            if let Some(proj_ids) = proyectos_por_docente.get(&m.id_docente) {
                for pid in proj_ids {
                    all_proyecto_ids.insert(pid.clone());
                }
            }
        }

        let mut proyecto_titles: Vec<String> = all_proyecto_ids
            .iter()
            .filter_map(|pid| proyectos.get(pid))
            .filter(|p| p.activo == 1)
            .map(|p| p.titulo_proyecto.clone())
            .collect();
        proyecto_titles.sort();

        data.push(ExportDataGrupo {
            grupo: grupo.nombre.clone(),
            descripcion: grupo.descripcion.clone(),
            coordinador,
            cantidad_miembros: miembros.len() as i64,
            miembros: if miembros_nombres.is_empty() { None } else { Some(miembros_nombres.join(" | ")) },
            lineas_investigacion: grupo.lineas_investigacion.clone(),
            cantidad_proyectos: proyecto_titles.len() as i64,
            proyectos: if proyecto_titles.is_empty() { None } else { Some(proyecto_titles.join(" | ")) },
        });
    }

    data.sort_by(|a, b| a.grupo.cmp(&b.grupo));
    Ok(data)
}

pub async fn get_data_exportacion_recursos(db: &Database) -> Result<Vec<ExportDataRecurso>, AppError> {
    let catalogo_map = data_loader::load_catalogos_map(db).await?;
    let docentes = data_loader::load_docentes_map(db).await?;
    let proyectos = data_loader::load_proyectos_map(db).await?;

    let patentes = db.collection::<Patente>("patentes")
        .find(doc! {})
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let productos = db.collection::<Producto>("productos")
        .find(doc! {})
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let equipamientos = db.collection::<Equipamiento>("equipamientos")
        .find(doc! {})
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let financiamientos = db.collection::<Financiamiento>("financiamientos")
        .find(doc! {})
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    fn resolve_label(catalogo_map: &HashMap<(String, String), CatalogoItem>, tipo_catalogo: &str, codigo: &Option<String>) -> Option<String> {
        codigo.as_ref().and_then(|c| catalogo_map.get(&(tipo_catalogo.to_string(), c.clone())).map(|i| i.nombre.clone()))
    }

    fn resolve_proyecto(proyectos: &HashMap<String, Proyecto>, proyecto_id: &Option<String>) -> Option<String> {
        proyecto_id.as_ref().and_then(|pid| proyectos.get(pid).map(|p| p.titulo_proyecto.clone()))
    }

    fn resolve_docente(docentes: &HashMap<String, Docente>, docente_id: &Option<String>) -> Option<String> {
        docente_id.as_ref().and_then(|did| docentes.get(did).map(|d| d.nombres_apellidos.clone()))
    }

    let mut data = Vec::new();

    for p in patentes {
        data.push(ExportDataRecurso {
            tipo_recurso: "Patente".to_string(),
            titulo_o_nombre: p.titulo.clone(),
            proyecto: resolve_proyecto(&proyectos, &p.proyecto_id),
            docente: resolve_docente(&docentes, &p.docente_id),
            tipo: resolve_label(&catalogo_map, "tipo_patente", &p.tipo),
            estado: resolve_label(&catalogo_map, "estado_patente", &p.estado),
            moneda: None,
            monto: None,
        });
    }

    for p in productos {
        data.push(ExportDataRecurso {
            tipo_recurso: "Producto".to_string(),
            titulo_o_nombre: p.nombre.clone(),
            proyecto: resolve_proyecto(&proyectos, &p.proyecto_id),
            docente: resolve_docente(&docentes, &p.docente_id),
            tipo: resolve_label(&catalogo_map, "tipo_producto", &p.tipo),
            estado: resolve_label(&catalogo_map, "etapa_producto", &p.etapa),
            moneda: None,
            monto: None,
        });
    }

    for e in equipamientos {
        data.push(ExportDataRecurso {
            tipo_recurso: "Equipamiento".to_string(),
            titulo_o_nombre: e.nombre.clone(),
            proyecto: resolve_proyecto(&proyectos, &e.proyecto_id),
            docente: None,
            tipo: None,
            estado: None,
            moneda: resolve_label(&catalogo_map, "moneda", &e.moneda),
            monto: e.valor_estimado,
        });
    }

    for f in financiamientos {
        data.push(ExportDataRecurso {
            tipo_recurso: "Financiamiento".to_string(),
            titulo_o_nombre: f.entidad_financiadora.clone(),
            proyecto: resolve_proyecto(&proyectos, &f.proyecto_id),
            docente: None,
            tipo: resolve_label(&catalogo_map, "tipo_financiamiento", &f.tipo),
            estado: resolve_label(&catalogo_map, "estado_financiero", &f.estado_financiero),
            moneda: resolve_label(&catalogo_map, "moneda", &f.moneda),
            monto: f.monto,
        });
    }

    data.sort_by(|a, b| a.tipo_recurso.cmp(&b.tipo_recurso).then_with(|| a.titulo_o_nombre.to_lowercase().cmp(&b.titulo_o_nombre.to_lowercase())));
    Ok(data)
}

pub async fn get_data_exportacion_docentes_perfil(db: &Database) -> Result<Vec<ExportDataDocentePerfil>, AppError> {
    let grados = data_loader::load_grados_map(db).await?;
    let grupos = data_loader::load_grupos_map(db).await?;
    let proyectos = data_loader::load_proyectos_map(db).await?;
    let participaciones = data_loader::load_participaciones(db).await?;

    let mut docentes = db.collection::<Docente>("docentes")
        .find(doc! {})
        .await?
        .try_collect::<Vec<_>>()
        .await?;
    docentes.sort_by(|a, b| a.nombres_apellidos.to_lowercase().cmp(&b.nombres_apellidos.to_lowercase()));

    let mut proyectos_por_docente: HashMap<String, Vec<String>> = HashMap::new();
    for p in &participaciones {
        proyectos_por_docente
            .entry(p.id_docente.clone())
            .or_default()
            .push(p.id_proyecto.clone());
    }

    let publicaciones = db.collection::<Publicacion>("publicaciones")
        .find(doc! {})
        .await?
        .try_collect::<Vec<_>>()
        .await?;
    let mut publicaciones_por_docente: HashMap<String, i64> = HashMap::new();
    for pub_item in &publicaciones {
        *publicaciones_por_docente.entry(pub_item.docente_id.clone()).or_default() += 1;
    }

    let mut data = Vec::new();
    for docente in docentes {
        let grado = grados
            .get(&docente.id_grado)
            .map(|g| g.nombre.clone())
            .unwrap_or_else(|| "Sin grado".to_string());

        let grupo_nombre = docente
            .grupo_investigacion_id
            .as_ref()
            .and_then(|gid| grupos.get(gid))
            .map(|g| g.nombre.clone());

        let proj_ids = proyectos_por_docente.get(&docente.id_docente);
        let mut proyecto_titles: Vec<String> = proj_ids
            .map(|ids| {
                ids.iter()
                    .filter_map(|pid| proyectos.get(pid))
                    .filter(|p| p.activo == 1)
                    .map(|p| p.titulo_proyecto.clone())
                    .collect()
            })
            .unwrap_or_default();
        proyecto_titles.sort();
        let cantidad_proyectos = proyecto_titles.len() as i64;

        let cantidad_publicaciones = publicaciones_por_docente.get(&docente.id_docente).copied().unwrap_or(0);

        data.push(ExportDataDocentePerfil {
            dni: docente.dni.clone(),
            nombres_apellidos: docente.nombres_apellidos.clone(),
            grado,
            renacyt_nivel: docente.renacyt_nivel.clone(),
            renacyt_grupo: docente.renacyt_grupo.clone(),
            renacyt_condicion: docente.renacyt_condicion.clone(),
            renacyt_orcid: docente.renacyt_orcid.clone(),
            grupo_investigacion: grupo_nombre,
            cantidad_proyectos,
            cantidad_publicaciones,
            proyectos: if proyecto_titles.is_empty() { None } else { Some(proyecto_titles.join(" | ")) },
            activo: docente.activo == 1,
        });
    }

    Ok(data)
}

pub async fn get_data_exportacion_proyectos_area(db: &Database) -> Result<Vec<ExportDataProyectoArea>, AppError> {
    let mut proyectos = db.collection::<Proyecto>("proyectos")
        .find(doc! { "activo": 1i64 })
        .await?
        .try_collect::<Vec<_>>()
        .await?;
    proyectos.sort_by(|a, b| a.titulo_proyecto.to_lowercase().cmp(&b.titulo_proyecto.to_lowercase()));

    let participaciones = data_loader::load_participaciones(db).await?;

    let mut docentes_por_proyecto: HashMap<String, HashSet<String>> = HashMap::new();
    for p in &participaciones {
        docentes_por_proyecto
            .entry(p.id_proyecto.clone())
            .or_default()
            .insert(p.id_docente.clone());
    }

    let mut areas: HashMap<String, (Vec<String>, HashSet<String>)> = HashMap::new();
    for proyecto in proyectos {
        let area_key = proyecto.campo_ocde.clone().unwrap_or_else(|| "Sin area OCDE".to_string());
        let entry = areas.entry(area_key).or_default();
        entry.0.push(proyecto.titulo_proyecto.clone());
        if let Some(docentes_set) = docentes_por_proyecto.get(&proyecto.id_proyecto) {
            for did in docentes_set {
                entry.1.insert(did.clone());
            }
        }
    }

    let mut data: Vec<ExportDataProyectoArea> = areas
        .into_iter()
        .map(|(area, (proyectos_list, docentes_set))| {
            ExportDataProyectoArea {
                area,
                cantidad_proyectos: proyectos_list.len() as i64,
                proyectos: if proyectos_list.is_empty() { None } else { Some(proyectos_list.join(" | ")) },
                cantidad_docentes: docentes_set.len() as i64,
            }
        })
        .collect();

    data.sort_by(|a, b| a.area.cmp(&b.area));
    Ok(data)
}

pub async fn get_proyectos_trend(db: &Database) -> Result<Vec<ProyectosTrendItem>, AppError> {
    let proyectos = db.collection::<Proyecto>("proyectos")
        .find(doc! { "activo": 1i64 })
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let mut trend: HashMap<(i32, u32), i64> = HashMap::new();

    for proj in &proyectos {
        let millis = proj.updated_at.unwrap_or(0);
        if millis == 0 {
            continue;
        }
        let total_months = millis / 2_628_000_000;
        let year = 1970 + (total_months / 12) as i32;
        let month = ((total_months % 12) + 1) as u32;
        if month > 12 {
            continue;
        }
        *trend.entry((year, month)).or_default() += 1;
    }

    let mut items: Vec<ProyectosTrendItem> = trend
        .into_iter()
        .map(|((anio, mes), cantidad)| ProyectosTrendItem { anio, mes, cantidad })
        .collect();
    items.sort_by(|a, b| a.anio.cmp(&b.anio).then_with(|| a.mes.cmp(&b.mes)));
    Ok(items)
}

pub async fn get_renacyt_distribucion(db: &Database) -> Result<Vec<RenacytDistribucionItem>, AppError> {
    let docentes = db.collection::<Docente>("docentes")
        .find(doc! { "activo": 1i64 })
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let proyectos = data_loader::load_proyectos_map(db).await?;
    let participaciones = data_loader::load_participaciones(db).await?;

    let mut docentes_con_proyectos: HashSet<String> = HashSet::new();
    for p in &participaciones {
        if let Some(proj) = proyectos.get(&p.id_proyecto) {
            if proj.activo == 1 {
                docentes_con_proyectos.insert(p.id_docente.clone());
            }
        }
    }

    let mut grupos: HashMap<String, RenacytDistribucionItem> = HashMap::new();
    for docente in &docentes {
        let nivel = docente
            .renacyt_nivel
            .as_deref()
            .filter(|v| !v.trim().is_empty())
            .unwrap_or("No registrado")
            .to_string();

        let entry = grupos.entry(nivel.clone()).or_insert(RenacytDistribucionItem {
            nivel,
            cantidad_docentes: 0,
            con_proyectos: 0,
            sin_proyectos: 0,
        });
        entry.cantidad_docentes += 1;
        if docentes_con_proyectos.contains(&docente.id_docente) {
            entry.con_proyectos += 1;
        } else {
            entry.sin_proyectos += 1;
        }
    }

    let mut items: Vec<RenacytDistribucionItem> = grupos.into_values().collect();
    items.sort_by(|a, b| a.nivel.to_lowercase().cmp(&b.nivel.to_lowercase()));
    Ok(items)
}
