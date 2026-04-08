use sqlx::{query, query_as, SqlitePool};
use crate::domain::proyecto::{
    Proyecto,
    CreateProyectoRequest,
    CreateProyectoConParticipantesRequest,
    ExportDataConProjectos,
    ProyectoDetalle,
    EliminarProyectoResultado,
};
use crate::domain::estadisticas::{DocenteProyectosCount, KpisDashboard, ExportData};
use crate::error::AppError;

pub async fn create_proyecto_con_participantes(pool: &SqlitePool, request: CreateProyectoConParticipantesRequest) -> Result<Proyecto, AppError> {
    let mut tx = pool.begin().await?;
    let proyecto = Proyecto::new(CreateProyectoRequest { titulo_proyecto: request.titulo_proyecto });
    query("INSERT INTO proyecto (id_proyecto, titulo_proyecto, activo) VALUES (?, ?, ?)")
        .bind(&proyecto.id_proyecto)
        .bind(&proyecto.titulo_proyecto)
        .bind(proyecto.activo)
        .execute(&mut *tx)
        .await?;

    for docente_id in request.docentes_ids {
        query("INSERT INTO participacion (id_proyecto, id_docente) VALUES (?, ?)")
            .bind(&proyecto.id_proyecto)
            .bind(&docente_id)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;
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
    let proyectos = query_as::<_, ProyectoDetalle>(
        r#"
        SELECT
            p.id_proyecto,
            p.titulo_proyecto,
            COUNT(pa.id_docente) as cantidad_docentes,
            GROUP_CONCAT(d.nombres_apellidos, ' | ') as docentes,
            p.activo as activo
        FROM proyecto p
        LEFT JOIN participacion pa ON p.id_proyecto = pa.id_proyecto
        LEFT JOIN docente d ON pa.id_docente = d.id_docente
        GROUP BY p.id_proyecto
        ORDER BY p.titulo_proyecto ASC
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(proyectos)
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