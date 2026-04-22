use sqlx::{query, query_as, Row, SqlitePool};
use std::collections::HashMap;
use crate::domain::proyecto::{
    Proyecto,
    CreateProyectoRequest,
    CreateProyectoConParticipantesRequest,
    UpdateProyectoConParticipantesRequest,
    ExportDataConProjectos,
    ProyectoParticipanteResumen,
    ProyectoDetalle,
    EliminarProyectoResultado,
};
use crate::domain::estadisticas::{DocenteProyectosCount, KpisDashboard, ExportData};
use crate::error::AppError;
use crate::services::proyecto_service;

async fn validate_docentes_activos(pool: &SqlitePool, docentes_ids: &[String]) -> Result<(), AppError> {
    for docente_id in docentes_ids {
        let docente_exists = query("SELECT id_docente FROM docente WHERE id_docente = ? AND activo = 1")
            .bind(docente_id)
            .fetch_optional(pool)
            .await?;

        if docente_exists.is_none() {
            return Err(AppError::InternalError(
                "Uno o más docentes seleccionados no existen o están inactivos.".to_string(),
            ));
        }
    }

    Ok(())
}

pub async fn create_proyecto_con_participantes(pool: &SqlitePool, request: CreateProyectoConParticipantesRequest) -> Result<Proyecto, AppError> {
    let prepared = proyecto_service::prepare_create_input(request)?;
    validate_docentes_activos(pool, &prepared.docentes_ids).await?;

    let mut tx = pool.begin().await?;
    let proyecto = Proyecto::new(CreateProyectoRequest { titulo_proyecto: prepared.titulo_proyecto });
    query("INSERT INTO proyecto (id_proyecto, titulo_proyecto, activo) VALUES (?, ?, ?)")
        .bind(&proyecto.id_proyecto)
        .bind(&proyecto.titulo_proyecto)
        .bind(proyecto.activo)
        .execute(&mut *tx)
        .await?;

    for docente_id in prepared.docentes_ids {
        query("INSERT INTO participacion (id_proyecto, id_docente, es_responsable) VALUES (?, ?, ?)")
            .bind(&proyecto.id_proyecto)
            .bind(&docente_id)
            .bind((prepared.docente_responsable_id.as_deref() == Some(docente_id.as_str())) as i64)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;
    Ok(proyecto)
}

pub async fn update_proyecto_con_participantes(
    pool: &SqlitePool,
    id_proyecto: &str,
    request: UpdateProyectoConParticipantesRequest,
) -> Result<Proyecto, AppError> {
    let prepared = proyecto_service::prepare_update_input(request)?;
    validate_docentes_activos(pool, &prepared.docentes_ids).await?;

    let mut tx = pool.begin().await?;

    let proyecto_exists = query("SELECT id_proyecto FROM proyecto WHERE id_proyecto = ?")
        .bind(id_proyecto)
        .fetch_optional(&mut *tx)
        .await?;

    if proyecto_exists.is_none() {
        return Err(AppError::NotFound("Proyecto no encontrado.".to_string()));
    }

    query("UPDATE proyecto SET titulo_proyecto = ? WHERE id_proyecto = ?")
        .bind(&prepared.titulo_proyecto)
        .bind(id_proyecto)
        .execute(&mut *tx)
        .await?;

    query("DELETE FROM participacion WHERE id_proyecto = ?")
        .bind(id_proyecto)
        .execute(&mut *tx)
        .await?;

    for docente_id in prepared.docentes_ids {
        query("INSERT INTO participacion (id_proyecto, id_docente, es_responsable) VALUES (?, ?, ?)")
            .bind(id_proyecto)
            .bind(&docente_id)
            .bind((prepared.docente_responsable_id.as_deref() == Some(docente_id.as_str())) as i64)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;

    let proyecto = query_as::<_, Proyecto>(
        "SELECT id_proyecto, titulo_proyecto, activo FROM proyecto WHERE id_proyecto = ?"
    )
    .bind(id_proyecto)
    .fetch_one(pool)
    .await?;

    Ok(proyecto)
}

pub async fn buscar_proyectos_por_docente(pool: &SqlitePool, id_docente: &str) -> Result<Vec<Proyecto>, AppError> {
    let proyectos = query_as::<_, Proyecto>(
        r#"
        SELECT p.id_proyecto, p.titulo_proyecto, p.activo
        FROM proyecto p
        INNER JOIN participacion pa ON p.id_proyecto = pa.id_proyecto
        WHERE pa.id_docente = ? AND p.activo = 1
        "#
    )
    .bind(id_docente)
    .fetch_all(pool)
    .await?;
    Ok(proyectos)
}

pub async fn get_all_proyectos_detalle(pool: &SqlitePool) -> Result<Vec<ProyectoDetalle>, AppError> {
    let proyectos = query_as::<_, Proyecto>(
        "SELECT id_proyecto, titulo_proyecto, activo FROM proyecto ORDER BY titulo_proyecto ASC"
    )
    .fetch_all(pool)
    .await?;

    let participantes_rows = query(
        r#"
        SELECT
            p.id_proyecto as id_proyecto,
            d.id_docente as id_docente,
            d.nombres_apellidos as nombre,
            COALESCE(g.nombre, 'Sin grado') as grado,
            COALESCE(d.renacyt_nivel, 'No registrado') as renacyt_nivel,
            COALESCE(pa.es_responsable, 0) as es_responsable
        FROM participacion pa
        INNER JOIN proyecto p ON pa.id_proyecto = p.id_proyecto
        INNER JOIN docente d ON pa.id_docente = d.id_docente
        LEFT JOIN grado_academico g ON d.id_grado = g.id_grado
        ORDER BY pa.es_responsable DESC, d.nombres_apellidos ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut participantes_por_proyecto: HashMap<String, Vec<ProyectoParticipanteResumen>> = HashMap::new();

    for row in participantes_rows {
        let id_proyecto: String = row.try_get("id_proyecto")?;
        let id_docente: String = row.try_get("id_docente")?;
        let nombre: String = row.try_get("nombre")?;
        let grado: String = row.try_get("grado")?;
        let renacyt_nivel: String = row.try_get("renacyt_nivel")?;
        let es_responsable = row.try_get::<i64, _>("es_responsable")? == 1;

        participantes_por_proyecto
            .entry(id_proyecto)
            .or_default()
            .push(ProyectoParticipanteResumen {
                id_docente,
                nombre,
                grado,
                renacyt_nivel,
                es_responsable,
            });
    }

    let detalles = proyectos
        .into_iter()
        .map(|proyecto| {
            let participantes = participantes_por_proyecto.remove(&proyecto.id_proyecto).unwrap_or_default();
            let cantidad_docentes = participantes.len() as i64;
            let docente_responsable = participantes
                .iter()
                .find(|participante| participante.es_responsable)
                .map(|participante| participante.nombre.clone());
            let docentes = if participantes.is_empty() {
                None
            } else {
                Some(participantes.iter().map(|participante| format!("{} ({} · {})", participante.nombre, participante.grado, participante.renacyt_nivel)).collect::<Vec<_>>().join(" | "))
            };
            let participantes_json = if participantes.is_empty() {
                None
            } else {
                serde_json::to_string(&participantes).ok()
            };

            ProyectoDetalle {
                id_proyecto: proyecto.id_proyecto,
                titulo_proyecto: proyecto.titulo_proyecto,
                cantidad_docentes,
                docente_responsable,
                docentes,
                participantes_json,
                activo: proyecto.activo,
            }
        })
        .collect();

    Ok(detalles)
}

pub async fn eliminar_relacion_proyecto_docente(
    pool: &SqlitePool,
    id_proyecto: &str,
    id_docente: &str,
) -> Result<(), AppError> {
    query("DELETE FROM participacion WHERE id_proyecto = ? AND id_docente = ?")
        .bind(id_proyecto)
        .bind(id_docente)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn eliminar_relaciones_proyecto(
    pool: &SqlitePool,
    id_proyecto: &str,
) -> Result<(), AppError> {
    query("DELETE FROM participacion WHERE id_proyecto = ?")
        .bind(id_proyecto)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn eliminar_proyecto(pool: &SqlitePool, id_proyecto: &str) -> Result<EliminarProyectoResultado, AppError> {
    let docentes_relacionados: (i64,) = query_as("SELECT COUNT(*) FROM participacion WHERE id_proyecto = ?")
        .bind(id_proyecto)
        .fetch_one(pool)
        .await?;

    if docentes_relacionados.0 > 0 {
        return Err(AppError::InternalError(
            "No se puede eliminar el proyecto porque aún tiene docentes relacionados. Elimine primero esas relaciones.".to_string(),
        ));
    }

    query("UPDATE proyecto SET activo = 0 WHERE id_proyecto = ?")
        .bind(id_proyecto)
        .execute(pool)
        .await?;

    Ok(EliminarProyectoResultado {
        accion: "desactivado".to_string(),
        mensaje: "Proyecto desactivado correctamente.".to_string(),
    })
}

pub async fn reactivar_proyecto(pool: &SqlitePool, id_proyecto: &str) -> Result<Proyecto, AppError> {
    query("UPDATE proyecto SET activo = 1 WHERE id_proyecto = ?")
        .bind(id_proyecto)
        .execute(pool)
        .await?;

    let proyecto = query_as::<_, Proyecto>(
        "SELECT id_proyecto, titulo_proyecto, activo FROM proyecto WHERE id_proyecto = ?"
    )
    .bind(id_proyecto)
    .fetch_one(pool)
    .await?;

    Ok(proyecto)
}

pub async fn get_estadisticas_proyectos_x_docente(pool: &SqlitePool) -> Result<Vec<DocenteProyectosCount>, AppError> {
    let stats = query_as::<_, DocenteProyectosCount>(
        r#"
        SELECT d.nombres_apellidos as nombre, COUNT(p.id_proyecto) as cantidad
        FROM docente d
        LEFT JOIN participacion pa ON d.id_docente = pa.id_docente
        LEFT JOIN proyecto p ON pa.id_proyecto = p.id_proyecto AND p.activo = 1
        WHERE d.activo = 1
        GROUP BY d.id_docente
        ORDER BY cantidad DESC
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(stats)
}

pub async fn get_kpis_dashboard(pool: &SqlitePool) -> Result<KpisDashboard, AppError> {
    let total_proyectos: (i64,) = query_as("SELECT COUNT(*) FROM proyecto WHERE activo = 1")
        .fetch_one(pool)
        .await?;
    let total_docentes: (i64,) = query_as("SELECT COUNT(*) FROM docente WHERE activo = 1")
        .fetch_one(pool)
        .await?;
    let docentes_con_1_proyecto: (i64,) = query_as(
        r#"
        SELECT COUNT(*) FROM (
            SELECT d.id_docente
            FROM docente d
            LEFT JOIN participacion pa ON d.id_docente = pa.id_docente
            LEFT JOIN proyecto p ON pa.id_proyecto = p.id_proyecto AND p.activo = 1
            WHERE d.activo = 1
            GROUP BY d.id_docente
            HAVING COUNT(p.id_proyecto) = 1
        )
        "#
    )
    .fetch_one(pool)
    .await?;
    let docentes_multiples_proyectos: (i64,) = query_as(
        r#"
        SELECT COUNT(*) FROM (
            SELECT d.id_docente
            FROM docente d
            LEFT JOIN participacion pa ON d.id_docente = pa.id_docente
            LEFT JOIN proyecto p ON pa.id_proyecto = p.id_proyecto AND p.activo = 1
            WHERE d.activo = 1
            GROUP BY d.id_docente
            HAVING COUNT(p.id_proyecto) > 1
        )
        "#
    )
    .fetch_one(pool)
    .await?;
    Ok(KpisDashboard {
        total_proyectos: total_proyectos.0,
        total_docentes: total_docentes.0,
        docentes_con_1_proyecto: docentes_con_1_proyecto.0,
        docentes_multiples_proyectos: docentes_multiples_proyectos.0,
    })
}

pub async fn get_data_exportacion_plana(pool: &SqlitePool) -> Result<Vec<ExportData>, AppError> {
    let data = query_as::<_, ExportData>(
        r#"
        SELECT
            p.titulo_proyecto as "proyecto",
            g.nombre as "grado",
            COALESCE(d.renacyt_nivel, 'No registrado') as "renacyt_nivel",
            d.nombres_apellidos as "docente",
            d.dni as "dni"
        FROM proyecto p
        INNER JOIN participacion pa ON p.id_proyecto = pa.id_proyecto
        INNER JOIN docente d ON pa.id_docente = d.id_docente
        INNER JOIN grado_academico g ON d.id_grado = g.id_grado
        WHERE p.activo = 1 AND d.activo = 1
        ORDER BY p.titulo_proyecto ASC, d.nombres_apellidos ASC
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(data)
}

pub async fn get_data_exportacion_agrupada_docente(pool: &SqlitePool) -> Result<Vec<ExportDataConProjectos>, AppError> {
    let data = query_as::<_, ExportDataConProjectos>(
        r#"
        SELECT
            d.nombres_apellidos as "docente",
            d.dni as "dni",
            g.nombre as "grado",
            COALESCE(d.renacyt_nivel, 'No registrado') as "renacyt_nivel",
            COUNT(p.id_proyecto) as "cantidad_proyectos",
            GROUP_CONCAT(p.titulo_proyecto, ' | ') as "proyectos"
        FROM docente d
        INNER JOIN grado_academico g ON d.id_grado = g.id_grado
        LEFT JOIN participacion pa ON d.id_docente = pa.id_docente
        LEFT JOIN proyecto p ON pa.id_proyecto = p.id_proyecto AND p.activo = 1
        WHERE d.activo = 1
        GROUP BY d.id_docente
        ORDER BY d.nombres_apellidos ASC
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(data)
}