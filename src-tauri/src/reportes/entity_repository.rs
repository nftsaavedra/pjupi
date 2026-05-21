use std::collections::HashMap;

use futures_util::TryStreamExt;
use mongodb::{bson::doc, Database};

use crate::catalogos::models::CatalogoItem;
use crate::docentes::models::{Docente, Publicacion};
use crate::proyectos::models::{ParticipacionRecord, Proyecto};
use crate::recursos::models::{Equipamiento, Financiamiento, Patente, Producto};
use crate::reportes::entity_reports::*;
use crate::shared::data_loader;
use crate::shared::error::AppError;

// ═══════════════════════════════════════════════════════════════════════════════
// Reporte Proyecto Integral
// ═══════════════════════════════════════════════════════════════════════════════

pub async fn build_reporte_proyecto_integral(
    db: &Database,
    id_proyecto: &str,
) -> Result<ReporteProyectoIntegral, AppError> {
    let catalogo_map = data_loader::load_catalogos_map(db).await?;
    let grados = data_loader::load_grados_map(db).await?;
    let grupos = data_loader::load_grupos_map(db).await?;

    let proyecto = db
        .collection::<Proyecto>("proyectos")
        .find_one(doc! { "id_proyecto": id_proyecto })
        .await?
        .ok_or_else(|| AppError::NotFound("Proyecto no encontrado.".to_string()))?;

    let cabecera = ProyectoCabeceraReporte {
        id_proyecto: proyecto.id_proyecto.clone(),
        titulo_proyecto: proyecto.titulo_proyecto.clone(),
        activo: proyecto.activo == 1,
        campo_ocde: proyecto.campo_ocde.clone(),
        programas_relacionados: proyecto.programas_relacionados.clone(),
        fecha_creacion: None,
        fecha_actualizacion: proyecto.updated_at.map(|t| t.to_string()),
    };

    let participaciones = db
        .collection::<ParticipacionRecord>("participaciones")
        .find(doc! { "id_proyecto": id_proyecto })
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let mut equipo: Vec<MiembroProyectoReporte> = Vec::new();
    for part in &participaciones {
        let docente = db
            .collection::<Docente>("docentes")
            .find_one(doc! { "id_docente": &part.id_docente })
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!("Docente {} no encontrado.", part.id_docente))
            })?;

        let grado_nombre = grados
            .get(&docente.id_grado)
            .map(|g| g.nombre.clone())
            .unwrap_or_default();

        let (grupo_nombre, grupo_id) = docente
            .grupo_investigacion_id
            .as_ref()
            .and_then(|gid| grupos.get(gid))
            .map(|g| (Some(g.nombre.clone()), Some(g.id_grupo.clone())))
            .unwrap_or((None, None));

        let publicaciones_count = db
            .collection::<Publicacion>("publicaciones")
            .count_documents(doc! { "docente_id": &docente.id_docente })
            .await? as i64;

        equipo.push(MiembroProyectoReporte {
            id_docente: docente.id_docente.clone(),
            dni: docente.dni.clone(),
            nombres_apellidos: docente.nombres_apellidos.clone(),
            nombres: docente.nombres.clone(),
            apellido_paterno: docente.apellido_paterno.clone(),
            apellido_materno: docente.apellido_materno.clone(),
            grado_nombre,
            grado_id: docente.id_grado.clone(),
            es_responsable: part.es_responsable,
            renacyt_codigo_registro: docente.renacyt_codigo_registro.clone(),
            renacyt_nivel: docente.renacyt_nivel.clone(),
            renacyt_grupo: docente.renacyt_grupo.clone(),
            renacyt_condicion: docente.renacyt_condicion.clone(),
            renacyt_orcid: docente.renacyt_orcid.clone(),
            renacyt_scopus_author_id: docente.renacyt_scopus_author_id.clone(),
            grupo_nombre,
            grupo_id,
            publicaciones_count,
        });
    }

    let total_docentes = equipo.len();

    let patentes_raw = db
        .collection::<Patente>("patentes")
        .find(doc! { "proyecto_id": id_proyecto })
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let total_patentes = patentes_raw.len();
    let patentes: Vec<PatenteConEtiquetas> = patentes_raw
        .iter()
        .map(|p| PatenteConEtiquetas::from_patente(p, &catalogo_map))
        .collect();

    let productos_raw = db
        .collection::<Producto>("productos")
        .find(doc! { "proyecto_id": id_proyecto })
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let total_productos = productos_raw.len();
    let productos: Vec<ProductoConEtiquetas> = productos_raw
        .iter()
        .map(|p| ProductoConEtiquetas::from_producto(p, &catalogo_map))
        .collect();

    let equipamientos_raw = db
        .collection::<Equipamiento>("equipamientos")
        .find(doc! { "proyecto_id": id_proyecto })
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let total_equipamientos = equipamientos_raw.len();
    let equipamientos: Vec<EquipamientoConEtiquetas> = equipamientos_raw
        .iter()
        .map(|e| EquipamientoConEtiquetas::from_equipamiento(e, &catalogo_map))
        .collect();

    let financiamientos_raw = db
        .collection::<Financiamiento>("financiamientos")
        .find(doc! { "proyecto_id": id_proyecto })
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let total_financiamientos = financiamientos_raw.len();
    let financiamientos: Vec<FinanciamientoConEtiquetas> = financiamientos_raw
        .iter()
        .map(|f| FinanciamientoConEtiquetas::from_financiamiento(f, &catalogo_map))
        .collect();

    let resumen_financiero = build_resumen_financiero(&financiamientos, &catalogo_map);

    Ok(ReporteProyectoIntegral {
        cabecera,
        equipo,
        total_docentes,
        patentes,
        total_patentes,
        productos,
        total_productos,
        equipamientos,
        total_equipamientos,
        financiamientos,
        total_financiamientos,
        resumen_financiero,
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
// Reporte Docente Integral
// ═══════════════════════════════════════════════════════════════════════════════

pub async fn build_reporte_docente_integral(
    db: &Database,
    id_docente: &str,
) -> Result<ReporteDocenteIntegral, AppError> {
    let catalogo_map = data_loader::load_catalogos_map(db).await?;
    let grados = data_loader::load_grados_map(db).await?;
    let grupos = data_loader::load_grupos_map(db).await?;
    let docentes_map = data_loader::load_docentes_map(db).await?;
    let proyectos_map = data_loader::load_proyectos_map(db).await?;
    let todas_participaciones = data_loader::load_participaciones(db).await?;

    let docente = db
        .collection::<Docente>("docentes")
        .find_one(doc! { "id_docente": id_docente })
        .await?
        .ok_or_else(|| AppError::NotFound("Docente no encontrado.".to_string()))?;

    let grado_nombre = grados
        .get(&docente.id_grado)
        .map(|g| g.nombre.clone())
        .unwrap_or_default();

    let (grupo_nombre, grupo_id) = docente
        .grupo_investigacion_id
        .as_ref()
        .and_then(|gid| grupos.get(gid))
        .map(|g| (Some(g.nombre.clone()), Some(g.id_grupo.clone())))
        .unwrap_or((None, None));

    let perfil = PerfilDocenteReporte {
        id_docente: docente.id_docente.clone(),
        dni: docente.dni.clone(),
        nombres_apellidos: docente.nombres_apellidos.clone(),
        nombres: docente.nombres.clone(),
        apellido_paterno: docente.apellido_paterno.clone(),
        apellido_materno: docente.apellido_materno.clone(),
        grado_nombre,
        grado_id: docente.id_grado.clone(),
        renacyt_codigo_registro: docente.renacyt_codigo_registro.clone(),
        renacyt_id_investigador: docente.renacyt_id_investigador.clone(),
        renacyt_nivel: docente.renacyt_nivel.clone(),
        renacyt_grupo: docente.renacyt_grupo.clone(),
        renacyt_condicion: docente.renacyt_condicion.clone(),
        renacyt_fecha_informe_calificacion: docente.renacyt_fecha_informe_calificacion,
        renacyt_fecha_registro: docente.renacyt_fecha_registro,
        renacyt_fecha_ultima_revision: docente.renacyt_fecha_ultima_revision,
        renacyt_orcid: docente.renacyt_orcid.clone(),
        renacyt_scopus_author_id: docente.renacyt_scopus_author_id.clone(),
        renacyt_ficha_url: docente.renacyt_ficha_url.clone(),
        renacyt_formaciones_academicas_json: docente.renacyt_formaciones_academicas_json.clone(),
        grupo_nombre,
        grupo_id,
    };

    let mis_participaciones: Vec<&ParticipacionRecord> = todas_participaciones
        .iter()
        .filter(|p| p.id_docente == id_docente)
        .collect();

    let mut proyectos_detalle: Vec<ProyectoDocenteDetalle> = Vec::new();
    let mut proyecto_ids: Vec<String> = Vec::new();

    for participacion in &mis_participaciones {
        let proyecto_id = &participacion.id_proyecto;
        proyecto_ids.push(proyecto_id.clone());

        let proyecto = match proyectos_map.get(proyecto_id) {
            Some(p) => p,
            None => continue,
        };

        let colegas: Vec<ColegaProyecto> = todas_participaciones
            .iter()
            .filter(|p| p.id_proyecto == *proyecto_id && p.id_docente != id_docente)
            .filter_map(|p| {
                docentes_map.get(&p.id_docente).map(|d| {
                    let colega_grado = grados
                        .get(&d.id_grado)
                        .map(|g| g.nombre.clone())
                        .unwrap_or_default();
                    ColegaProyecto {
                        id_docente: d.id_docente.clone(),
                        nombres_apellidos: d.nombres_apellidos.clone(),
                        grado_nombre: colega_grado,
                        es_responsable: p.es_responsable,
                    }
                })
            })
            .collect();

        let patentes_count = db
            .collection::<Patente>("patentes")
            .count_documents(doc! { "proyecto_id": proyecto_id })
            .await? as usize;
        let productos_count = db
            .collection::<Producto>("productos")
            .count_documents(doc! { "proyecto_id": proyecto_id })
            .await? as usize;
        let equipamientos_count = db
            .collection::<Equipamiento>("equipamientos")
            .count_documents(doc! { "proyecto_id": proyecto_id })
            .await? as usize;
        let financiamientos_count = db
            .collection::<Financiamiento>("financiamientos")
            .count_documents(doc! { "proyecto_id": proyecto_id })
            .await? as usize;

        proyectos_detalle.push(ProyectoDocenteDetalle {
            id_proyecto: proyecto.id_proyecto.clone(),
            titulo_proyecto: proyecto.titulo_proyecto.clone(),
            es_responsable: participacion.es_responsable,
            activo: proyecto.activo == 1,
            campo_ocde: proyecto.campo_ocde.clone(),
            programas_relacionados: proyecto.programas_relacionados.clone(),
            colegas,
            recursos_en_proyecto: RecursosProyectoResumen {
                patentes: patentes_count,
                productos: productos_count,
                equipamientos: equipamientos_count,
                financiamientos: financiamientos_count,
            },
        });
    }

    let total_proyectos = proyectos_detalle.len();

    let patentes_raw = db
        .collection::<Patente>("patentes")
        .find(doc! { "docente_id": id_docente })
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let total_patentes = patentes_raw.len();
    let patentes: Vec<PatenteConEtiquetas> = patentes_raw
        .iter()
        .map(|p| PatenteConEtiquetas::from_patente(p, &catalogo_map))
        .collect();

    let productos_raw = db
        .collection::<Producto>("productos")
        .find(doc! { "docente_id": id_docente })
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let total_productos = productos_raw.len();
    let productos: Vec<ProductoConEtiquetas> = productos_raw
        .iter()
        .map(|p| ProductoConEtiquetas::from_producto(p, &catalogo_map))
        .collect();

    let equipamientos_raw = if proyecto_ids.is_empty() {
        Vec::new()
    } else {
        db.collection::<Equipamiento>("equipamientos")
            .find(doc! { "proyecto_id": { "$in": &proyecto_ids } })
            .await?
            .try_collect::<Vec<_>>()
            .await?
    };

    let total_equipamientos = equipamientos_raw.len();
    let equipamientos: Vec<EquipamientoConEtiquetas> = equipamientos_raw
        .iter()
        .map(|e| EquipamientoConEtiquetas::from_equipamiento(e, &catalogo_map))
        .collect();

    let publicaciones_raw = db
        .collection::<Publicacion>("publicaciones")
        .find(doc! { "docente_id": id_docente })
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let total_publicaciones = publicaciones_raw.len();
    let publicaciones: Vec<PublicacionConEtiquetas> = publicaciones_raw
        .iter()
        .map(|p| PublicacionConEtiquetas {
            id_publicacion: p.id_publicacion.clone(),
            pure_uuid: p.pure_uuid.clone(),
            titulo: p.titulo.clone(),
            tipo_publicacion: p.tipo_publicacion.clone(),
            doi: p.doi.clone(),
            scopus_eid: p.scopus_eid.clone(),
            anio_publicacion: p.anio_publicacion,
            autores_json: p.autores_json.clone(),
            estado_publicacion: p.estado_publicacion.clone(),
            journal_titulo: p.journal_titulo.clone(),
            issn: p.issn.clone(),
            pure_sincronizado_at: p.pure_sincronizado_at,
        })
        .collect();

    let trazabilidad = TrazabilidadDocente {
        updated_at: docente.updated_at,
        fecha_ultima_sincronizacion_renacyt: docente.renacyt_fecha_ultima_sincronizacion,
        fecha_ultima_sincronizacion_pure: None,
    };

    Ok(ReporteDocenteIntegral {
        perfil,
        proyectos: proyectos_detalle,
        total_proyectos,
        recursos: RecursosDocenteResumen {
            patentes,
            productos,
            equipamientos,
            total_patentes,
            total_productos,
            total_equipamientos,
        },
        publicaciones,
        total_publicaciones,
        trazabilidad,
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
// Reportes Docentes Integral (todos los activos)
// ═══════════════════════════════════════════════════════════════════════════════

pub async fn build_reportes_docentes_integral(
    db: &Database,
) -> Result<Vec<ReporteDocenteIntegral>, AppError> {
    let docentes = db
        .collection::<Docente>("docentes")
        .find(doc! { "activo": 1i64 })
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let mut reportes = Vec::with_capacity(docentes.len());
    for docente in docentes {
        let reporte = build_reporte_docente_integral(db, &docente.id_docente).await?;
        reportes.push(reporte);
    }

    Ok(reportes)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Helpers
// ═══════════════════════════════════════════════════════════════════════════════

fn build_resumen_financiero(
    financiamientos: &[FinanciamientoConEtiquetas],
    catalogo_map: &HashMap<(String, String), CatalogoItem>,
) -> ResumenFinanciero {
    let total_financiamientos = financiamientos.len();

    let mut moneda_groups: HashMap<String, MonedaDesglose> = HashMap::new();
    let mut estado_groups: HashMap<String, EstadoDesglose> = HashMap::new();

    for f in financiamientos {
        let moneda_key = f
            .moneda_codigo
            .clone()
            .unwrap_or_else(|| "sin_moneda".to_string());
        let moneda_nombre = f
            .moneda_nombre
            .clone()
            .or_else(|| {
                f.moneda_codigo
                    .as_ref()
                    .and_then(|c| catalogo_map.get(&("moneda".to_string(), c.clone())))
                    .map(|i| i.nombre.clone())
            })
            .unwrap_or_else(|| "Sin moneda".to_string());

        let entry = moneda_groups.entry(moneda_key.clone()).or_insert_with(|| {
            MonedaDesglose {
                moneda_codigo: moneda_key.clone(),
                moneda_nombre: moneda_nombre.clone(),
                cantidad: 0,
                monto_total: 0.0,
            }
        });
        entry.cantidad += 1;
        entry.monto_total += f.monto.unwrap_or(0.0);

        let estado_key = f
            .estado_financiero_codigo
            .clone()
            .unwrap_or_else(|| "sin_estado".to_string());
        let estado_nombre = f
            .estado_financiero_nombre
            .clone()
            .or_else(|| {
                f.estado_financiero_codigo
                    .as_ref()
                    .and_then(|c| {
                        catalogo_map.get(&("estado_financiero".to_string(), c.clone()))
                    })
                    .map(|i| i.nombre.clone())
            })
            .unwrap_or_else(|| "Sin estado".to_string());

        estado_groups
            .entry(estado_key.clone())
            .or_insert_with(|| EstadoDesglose {
                estado_codigo: estado_key.clone(),
                estado_nombre: estado_nombre.clone(),
                cantidad: 0,
            })
            .cantidad += 1;
    }

    let mut desglose_por_moneda: Vec<MonedaDesglose> = moneda_groups.into_values().collect();
    desglose_por_moneda.sort_by(|a, b| a.moneda_codigo.cmp(&b.moneda_codigo));

    let mut desglose_por_estado: Vec<EstadoDesglose> = estado_groups.into_values().collect();
    desglose_por_estado.sort_by(|a, b| a.estado_codigo.cmp(&b.estado_codigo));

    ResumenFinanciero {
        total_financiamientos,
        desglose_por_moneda,
        desglose_por_estado,
    }
}
