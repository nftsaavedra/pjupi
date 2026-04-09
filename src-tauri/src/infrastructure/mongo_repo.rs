use std::collections::{HashMap, HashSet};

use futures_util::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    options::IndexOptions,
    Database,
    IndexModel,
};
use rand_core::OsRng;

use crate::domain::docente::{CreateDocenteRequest, Docente, DocenteDetalle, EliminarDocenteResultado};
use crate::domain::estadisticas::{DocenteProyectosCount, ExportData, KpisDashboard};
use crate::domain::grado::{CreateGradoRequest, EliminarGradoResultado, GradoAcademico};
use crate::domain::proyecto::{
    CreateProyectoConParticipantesRequest,
    CreateProyectoRequest,
    EliminarProyectoResultado,
    ExportDataConProjectos,
    Proyecto,
    ProyectoDetalle,
};
use crate::domain::usuario::{AuthStatus, BootstrapUsuarioRequest, CreateUsuarioRequest, LoginUsuarioRequest, UpdateUsuarioRequest, Usuario, UsuarioConPassword};
use crate::error::AppError;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct ParticipacionRecord {
    #[serde(rename = "_id")]
    id: String,
    id_proyecto: String,
    id_docente: String,
}

pub async fn ensure_indexes(db: &Database) -> Result<(), AppError> {
    db.collection::<Document>("grados")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "id_grado": 1 })
                .options(Some(IndexOptions::builder().unique(true).build()))
                .build(),
        )
        .await?;
    db.collection::<Document>("grados")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "nombre": 1 })
                .options(Some(IndexOptions::builder().unique(true).build()))
                .build(),
        )
        .await?;

    db.collection::<Document>("docentes")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "id_docente": 1 })
                .options(Some(IndexOptions::builder().unique(true).build()))
                .build(),
        )
        .await?;
    db.collection::<Document>("docentes")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "dni": 1 })
                .options(Some(IndexOptions::builder().unique(true).build()))
                .build(),
        )
        .await?;
    db.collection::<Document>("docentes")
        .create_index(IndexModel::builder().keys(doc! { "renacyt_id_investigador": 1 }).build())
        .await?;
    db.collection::<Document>("docentes")
        .create_index(IndexModel::builder().keys(doc! { "renacyt_codigo_registro": 1 }).build())
        .await?;

    db.collection::<Document>("proyectos")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "id_proyecto": 1 })
                .options(Some(IndexOptions::builder().unique(true).build()))
                .build(),
        )
        .await?;

    db.collection::<Document>("participaciones")
        .create_index(IndexModel::builder().keys(doc! { "id_proyecto": 1 }).build())
        .await?;
    db.collection::<Document>("participaciones")
        .create_index(IndexModel::builder().keys(doc! { "id_docente": 1 }).build())
        .await?;

    db.collection::<Document>("usuarios")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "id_usuario": 1 })
                .options(Some(IndexOptions::builder().unique(true).build()))
                .build(),
        )
        .await?;
    db.collection::<Document>("usuarios")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "username": 1 })
                .options(Some(IndexOptions::builder().unique(true).build()))
                .build(),
        )
        .await?;

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

pub async fn create_grado(db: &Database, request: CreateGradoRequest) -> Result<GradoAcademico, AppError> {
    let grado = GradoAcademico::new(request);
    db.collection::<GradoAcademico>("grados").insert_one(&grado).await?;
    Ok(grado)
}

pub async fn get_all_grados(db: &Database) -> Result<Vec<GradoAcademico>, AppError> {
    let mut grados = db.collection::<GradoAcademico>("grados")
        .find(doc! {})
        .await?
        .try_collect::<Vec<_>>()
        .await?;
    grados.sort_by(|a, b| a.nombre.to_lowercase().cmp(&b.nombre.to_lowercase()));
    Ok(grados)
}

pub async fn get_grado_by_id(db: &Database, id_grado: &str) -> Result<GradoAcademico, AppError> {
    db.collection::<GradoAcademico>("grados")
        .find_one(doc! { "id_grado": id_grado })
        .await?
        .ok_or_else(|| AppError::NotFound("Grado no encontrado.".to_string()))
}

pub async fn update_grado(db: &Database, id_grado: &str, request: CreateGradoRequest) -> Result<GradoAcademico, AppError> {
    db.collection::<GradoAcademico>("grados")
        .update_one(
            doc! { "id_grado": id_grado },
            doc! { "$set": { "nombre": request.nombre, "descripcion": request.descripcion } },
        )
        .await?;
    get_grado_by_id(db, id_grado).await
}

pub async fn delete_grado(db: &Database, id_grado: &str) -> Result<EliminarGradoResultado, AppError> {
    let docentes_relacionados = db.collection::<Document>("docentes")
        .count_documents(doc! { "id_grado": id_grado })
        .await?;

    if docentes_relacionados > 0 {
        db.collection::<Document>("grados")
            .update_one(doc! { "id_grado": id_grado }, doc! { "$set": { "activo": 0i64 } })
            .await?;
        return Ok(EliminarGradoResultado {
            accion: "desactivado".to_string(),
            mensaje: "El grado está relacionado con docentes. Se desactivó en lugar de eliminarse.".to_string(),
        });
    }

    db.collection::<Document>("grados")
        .delete_one(doc! { "id_grado": id_grado })
        .await?;

    Ok(EliminarGradoResultado {
        accion: "eliminado".to_string(),
        mensaje: "Grado eliminado correctamente.".to_string(),
    })
}

pub async fn reactivar_grado(db: &Database, id_grado: &str) -> Result<GradoAcademico, AppError> {
    db.collection::<Document>("grados")
        .update_one(doc! { "id_grado": id_grado }, doc! { "$set": { "activo": 1i64 } })
        .await?;
    get_grado_by_id(db, id_grado).await
}

pub async fn create_docente(db: &Database, request: CreateDocenteRequest) -> Result<Docente, AppError> {
    let grado_existente = db.collection::<Document>("grados")
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
            }
        })
        .collect();

    Ok(detalles)
}

pub async fn delete_docente(db: &Database, id_docente: &str) -> Result<EliminarDocenteResultado, AppError> {
    let participaciones = db.collection::<Document>("participaciones")
        .count_documents(doc! { "id_docente": id_docente })
        .await?;

    db.collection::<Document>("docentes")
        .update_one(doc! { "id_docente": id_docente }, doc! { "$set": { "activo": 0i64 } })
        .await?;

    if participaciones > 0 {
        return Ok(EliminarDocenteResultado {
            accion: "desactivado".to_string(),
            mensaje: "Docente desactivado. Mantiene trazabilidad porque tiene proyectos relacionados.".to_string(),
        });
    }

    Ok(EliminarDocenteResultado {
        accion: "desactivado".to_string(),
        mensaje: "Docente desactivado correctamente.".to_string(),
    })
}

pub async fn reactivar_docente(db: &Database, id_docente: &str) -> Result<Docente, AppError> {
    db.collection::<Document>("docentes")
        .update_one(doc! { "id_docente": id_docente }, doc! { "$set": { "activo": 1i64 } })
        .await?;

    db.collection::<Docente>("docentes")
        .find_one(doc! { "id_docente": id_docente })
        .await?
        .ok_or_else(|| AppError::NotFound("Docente no encontrado.".to_string()))
}

pub async fn create_proyecto_con_participantes(db: &Database, request: CreateProyectoConParticipantesRequest) -> Result<Proyecto, AppError> {
    let docentes_activos = db.collection::<Docente>("docentes")
        .find(doc! { "id_docente": { "$in": &request.docentes_ids }, "activo": 1i64 })
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    if docentes_activos.len() != request.docentes_ids.len() {
        return Err(AppError::InternalError("Uno o más docentes seleccionados no existen o están inactivos.".to_string()));
    }

    let proyecto = Proyecto::new(CreateProyectoRequest { titulo_proyecto: request.titulo_proyecto });
    db.collection::<Proyecto>("proyectos").insert_one(&proyecto).await?;

    let participaciones_collection = db.collection::<ParticipacionRecord>("participaciones");
    for docente_id in request.docentes_ids {
        participaciones_collection.insert_one(ParticipacionRecord {
            id: format!("{}:{}", proyecto.id_proyecto, docente_id),
            id_proyecto: proyecto.id_proyecto.clone(),
            id_docente: docente_id,
        }).await?;
    }

    Ok(proyecto)
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
    let participaciones = load_participaciones(db).await?;

    let mut docentes_por_proyecto: HashMap<String, Vec<String>> = HashMap::new();
    for participacion in participaciones {
        if let Some(docente) = docentes.get(&participacion.id_docente) {
            docentes_por_proyecto
                .entry(participacion.id_proyecto)
                .or_default()
                .push(docente.nombres_apellidos.clone());
        }
    }

    let detalles = proyectos
        .into_iter()
        .map(|proyecto| {
            let docentes_proyecto = docentes_por_proyecto.remove(&proyecto.id_proyecto).unwrap_or_default();
            ProyectoDetalle {
                id_proyecto: proyecto.id_proyecto,
                titulo_proyecto: proyecto.titulo_proyecto,
                cantidad_docentes: docentes_proyecto.len() as i64,
                docentes: if docentes_proyecto.is_empty() {
                    None
                } else {
                    Some(docentes_proyecto.join(" | "))
                },
                activo: proyecto.activo,
            }
        })
        .collect();

    Ok(detalles)
}

pub async fn eliminar_relacion_proyecto_docente(db: &Database, id_proyecto: &str, id_docente: &str) -> Result<(), AppError> {
    let relation_id = format!("{}:{}", id_proyecto, id_docente);
    db.collection::<Document>("participaciones")
        .delete_one(doc! { "_id": relation_id })
        .await?;
    Ok(())
}

pub async fn eliminar_relaciones_proyecto(db: &Database, id_proyecto: &str) -> Result<(), AppError> {
    db.collection::<Document>("participaciones")
        .delete_many(doc! { "id_proyecto": id_proyecto })
        .await?;
    Ok(())
}

pub async fn eliminar_proyecto(db: &Database, id_proyecto: &str) -> Result<EliminarProyectoResultado, AppError> {
    let docentes_relacionados = db.collection::<Document>("participaciones")
        .count_documents(doc! { "id_proyecto": id_proyecto })
        .await?;

    if docentes_relacionados > 0 {
        return Err(AppError::InternalError(
            "No se puede eliminar el proyecto porque aún tiene docentes relacionados. Elimine primero esas relaciones.".to_string(),
        ));
    }

    db.collection::<Document>("proyectos")
        .update_one(doc! { "id_proyecto": id_proyecto }, doc! { "$set": { "activo": 0i64 } })
        .await?;

    Ok(EliminarProyectoResultado {
        accion: "desactivado".to_string(),
        mensaje: "Proyecto desactivado correctamente.".to_string(),
    })
}

pub async fn reactivar_proyecto(db: &Database, id_proyecto: &str) -> Result<Proyecto, AppError> {
    db.collection::<Document>("proyectos")
        .update_one(doc! { "id_proyecto": id_proyecto }, doc! { "$set": { "activo": 1i64 } })
        .await?;

    db.collection::<Proyecto>("proyectos")
        .find_one(doc! { "id_proyecto": id_proyecto })
        .await?
        .ok_or_else(|| AppError::NotFound("Proyecto no encontrado.".to_string()))
}

pub async fn get_estadisticas_proyectos_x_docente(db: &Database) -> Result<Vec<DocenteProyectosCount>, AppError> {
    let docentes = get_all_docentes(db).await?;
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
    let docentes = get_all_docentes(db).await?;
    let proyectos = db.collection::<Document>("proyectos")
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
            docente: docente.nombres_apellidos.clone(),
            dni: docente.dni.clone(),
        });
    }

    data.sort_by(|a, b| a.proyecto.cmp(&b.proyecto).then_with(|| a.docente.cmp(&b.docente)));
    Ok(data)
}

pub async fn get_data_exportacion_agrupada_docente(db: &Database) -> Result<Vec<ExportDataConProjectos>, AppError> {
    let grados = load_grados_map(db).await?;
    let docentes_activos = get_all_docentes(db).await?;
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

async fn load_usuarios(db: &Database) -> Result<Vec<UsuarioConPassword>, AppError> {
    db.collection::<UsuarioConPassword>("usuarios")
        .find(doc! {})
        .await?
        .try_collect::<Vec<_>>()
        .await
        .map_err(Into::into)
}

async fn count_usuarios(db: &Database) -> Result<u64, AppError> {
    db.collection::<Document>("usuarios")
        .count_documents(doc! {})
        .await
        .map_err(Into::into)
}

async fn get_usuario_by_username(db: &Database, username: &str) -> Result<UsuarioConPassword, AppError> {
    db.collection::<UsuarioConPassword>("usuarios")
        .find_one(doc! { "username": username.trim().to_lowercase() })
        .await?
        .ok_or_else(|| AppError::NotFound("Usuario no encontrado.".to_string()))
}

fn validar_usuario(username: &str, nombre_completo: &str, rol: &str) -> Result<(), AppError> {
    if username.trim().is_empty() || nombre_completo.trim().is_empty() || rol.trim().is_empty() {
        return Err(AppError::InternalError("Complete todos los campos del usuario.".to_string()));
    }
    if !matches!(rol.trim(), "admin" | "operador" | "consulta") {
        return Err(AppError::InternalError("El rol del usuario no es válido.".to_string()));
    }
    Ok(())
}

fn hash_password(password: &str) -> Result<String, AppError> {
    use argon2::{password_hash::{PasswordHasher, SaltString}, Argon2};

    if password.trim().len() < 8 {
        return Err(AppError::InternalError("La contraseña debe tener al menos 8 caracteres.".to_string()));
    }

    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|err| AppError::InternalError(format!("No se pudo proteger la contraseña: {err}")))
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    use argon2::{password_hash::PasswordHash, password_hash::PasswordVerifier, Argon2};

    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|err| AppError::InternalError(format!("No se pudo leer la contraseña protegida: {err}")))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub async fn get_auth_status(db: &Database) -> Result<AuthStatus, AppError> {
    let total = count_usuarios(db).await?;
    Ok(AuthStatus {
        has_users: total > 0,
        requires_setup: total == 0,
    })
}

pub async fn create_usuario(db: &Database, request: CreateUsuarioRequest) -> Result<Usuario, AppError> {
    validar_usuario(&request.username, &request.nombre_completo, &request.rol)?;
    let password_hash = hash_password(&request.password)?;
    let usuario = UsuarioConPassword::new(request, password_hash);
    db.collection::<UsuarioConPassword>("usuarios").insert_one(&usuario).await?;
    Ok(usuario.public_view())
}

pub async fn bootstrap_admin(db: &Database, request: BootstrapUsuarioRequest) -> Result<Usuario, AppError> {
    if count_usuarios(db).await? > 0 {
        return Err(AppError::InternalError("La configuración inicial ya fue completada.".to_string()));
    }

    create_usuario(
        db,
        CreateUsuarioRequest {
            username: request.username,
            nombre_completo: request.nombre_completo,
            rol: "admin".to_string(),
            password: request.password,
        },
    )
    .await
}

pub async fn login_usuario(db: &Database, request: LoginUsuarioRequest) -> Result<Usuario, AppError> {
    let usuario = get_usuario_by_username(db, &request.username).await?;

    if usuario.activo == 0 {
        return Err(AppError::InternalError("El usuario está inactivo.".to_string()));
    }

    if !verify_password(&request.password, &usuario.password_hash)? {
        return Err(AppError::InternalError("Usuario o contraseña incorrectos.".to_string()));
    }

    Ok(usuario.public_view())
}

pub async fn get_all_usuarios(db: &Database) -> Result<Vec<Usuario>, AppError> {
    let mut usuarios: Vec<Usuario> = load_usuarios(db)
        .await?
        .into_iter()
        .map(|usuario| usuario.public_view())
        .collect();
    usuarios.sort_by(|a, b| a.username.cmp(&b.username));
    Ok(usuarios)
}

pub async fn update_usuario(db: &Database, id_usuario: &str, request: UpdateUsuarioRequest) -> Result<Usuario, AppError> {
    validar_usuario(&request.username, &request.nombre_completo, &request.rol)?;

    let mut updates = doc! {
        "username": request.username.trim().to_lowercase(),
        "nombre_completo": request.nombre_completo.trim(),
        "rol": request.rol.trim(),
    };

    if let Some(password) = request.password.as_deref().filter(|value| !value.trim().is_empty()) {
        updates.insert("password_hash", hash_password(password)?);
    }

    db.collection::<Document>("usuarios")
        .update_one(doc! { "id_usuario": id_usuario }, doc! { "$set": updates })
        .await?;

    let usuario = db.collection::<UsuarioConPassword>("usuarios")
        .find_one(doc! { "id_usuario": id_usuario })
        .await?
        .ok_or_else(|| AppError::NotFound("Usuario no encontrado.".to_string()))?;
    Ok(usuario.public_view())
}

pub async fn desactivar_usuario(db: &Database, id_usuario: &str) -> Result<Usuario, AppError> {
    db.collection::<Document>("usuarios")
        .update_one(doc! { "id_usuario": id_usuario }, doc! { "$set": { "activo": 0i64 } })
        .await?;

    let usuario = db.collection::<UsuarioConPassword>("usuarios")
        .find_one(doc! { "id_usuario": id_usuario })
        .await?
        .ok_or_else(|| AppError::NotFound("Usuario no encontrado.".to_string()))?;
    Ok(usuario.public_view())
}

pub async fn reactivar_usuario(db: &Database, id_usuario: &str) -> Result<Usuario, AppError> {
    db.collection::<Document>("usuarios")
        .update_one(doc! { "id_usuario": id_usuario }, doc! { "$set": { "activo": 1i64 } })
        .await?;

    let usuario = db.collection::<UsuarioConPassword>("usuarios")
        .find_one(doc! { "id_usuario": id_usuario })
        .await?
        .ok_or_else(|| AppError::NotFound("Usuario no encontrado.".to_string()))?;
    Ok(usuario.public_view())
}