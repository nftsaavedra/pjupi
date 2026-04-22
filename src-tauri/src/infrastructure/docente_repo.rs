use sqlx::{query, query_as, SqlitePool};
use crate::domain::docente::{Docente, CreateDocenteRequest, DocenteDetalle, EliminarDocenteResultado};
use crate::error::AppError;
use crate::services::docente_service;

pub async fn create_docente(pool: &SqlitePool, request: CreateDocenteRequest) -> Result<Docente, AppError> {
    let docente = Docente::new(request);
    query("INSERT INTO docente (id_docente, dni, id_grado, nombres_apellidos, nombres, apellido_paterno, apellido_materno, activo, renacyt_codigo_registro, renacyt_id_investigador, renacyt_nivel, renacyt_grupo, renacyt_condicion, renacyt_fecha_informe_calificacion, renacyt_fecha_registro, renacyt_fecha_ultima_revision, renacyt_orcid, renacyt_scopus_author_id, renacyt_fecha_ultima_sincronizacion, renacyt_ficha_url, renacyt_formaciones_academicas_json) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(&docente.id_docente)
        .bind(&docente.dni)
        .bind(&docente.id_grado)
        .bind(&docente.nombres_apellidos)
        .bind(&docente.nombres)
        .bind(&docente.apellido_paterno)
        .bind(&docente.apellido_materno)
        .bind(docente.activo)
        .bind(&docente.renacyt_codigo_registro)
        .bind(&docente.renacyt_id_investigador)
        .bind(&docente.renacyt_nivel)
        .bind(&docente.renacyt_grupo)
        .bind(&docente.renacyt_condicion)
        .bind(docente.renacyt_fecha_informe_calificacion)
        .bind(docente.renacyt_fecha_registro)
        .bind(docente.renacyt_fecha_ultima_revision)
        .bind(&docente.renacyt_orcid)
        .bind(&docente.renacyt_scopus_author_id)
        .bind(docente.renacyt_fecha_ultima_sincronizacion)
        .bind(&docente.renacyt_ficha_url)
        .bind(&docente.renacyt_formaciones_academicas_json)
        .execute(pool)
        .await?;
    Ok(docente)
}

pub async fn get_all_docentes(pool: &SqlitePool) -> Result<Vec<Docente>, AppError> {
    let docentes = query_as::<_, Docente>(
        "SELECT id_docente, dni, id_grado, nombres_apellidos, nombres, apellido_paterno, apellido_materno, activo, renacyt_codigo_registro, renacyt_id_investigador, renacyt_nivel, renacyt_grupo, renacyt_condicion, renacyt_fecha_informe_calificacion, renacyt_fecha_registro, renacyt_fecha_ultima_revision, renacyt_orcid, renacyt_scopus_author_id, renacyt_fecha_ultima_sincronizacion, renacyt_ficha_url, renacyt_formaciones_academicas_json FROM docente WHERE activo = 1"
    )
        .fetch_all(pool)
        .await?;
    Ok(docentes)
}

pub async fn get_docente_by_dni(pool: &SqlitePool, dni: &str) -> Result<Option<Docente>, AppError> {
    let docente = query_as::<_, Docente>(
        "SELECT id_docente, dni, id_grado, nombres_apellidos, nombres, apellido_paterno, apellido_materno, activo, renacyt_codigo_registro, renacyt_id_investigador, renacyt_nivel, renacyt_grupo, renacyt_condicion, renacyt_fecha_informe_calificacion, renacyt_fecha_registro, renacyt_fecha_ultima_revision, renacyt_orcid, renacyt_scopus_author_id, renacyt_fecha_ultima_sincronizacion, renacyt_ficha_url, renacyt_formaciones_academicas_json FROM docente WHERE dni = ?"
    )
    .bind(dni)
    .fetch_optional(pool)
    .await?;

    Ok(docente)
}

pub async fn get_docente_by_id(pool: &SqlitePool, id_docente: &str) -> Result<Docente, AppError> {
    query_as::<_, Docente>(
        "SELECT id_docente, dni, id_grado, nombres_apellidos, nombres, apellido_paterno, apellido_materno, activo, renacyt_codigo_registro, renacyt_id_investigador, renacyt_nivel, renacyt_grupo, renacyt_condicion, renacyt_fecha_informe_calificacion, renacyt_fecha_registro, renacyt_fecha_ultima_revision, renacyt_orcid, renacyt_scopus_author_id, renacyt_fecha_ultima_sincronizacion, renacyt_ficha_url, renacyt_formaciones_academicas_json FROM docente WHERE id_docente = ?"
    )
    .bind(id_docente)
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}

pub async fn update_docente_renacyt(pool: &SqlitePool, docente: &Docente) -> Result<(), AppError> {
    query(
        "UPDATE docente SET renacyt_codigo_registro = ?, renacyt_id_investigador = ?, renacyt_nivel = ?, renacyt_grupo = ?, renacyt_condicion = ?, renacyt_fecha_informe_calificacion = ?, renacyt_fecha_registro = ?, renacyt_fecha_ultima_revision = ?, renacyt_orcid = ?, renacyt_scopus_author_id = ?, renacyt_fecha_ultima_sincronizacion = ?, renacyt_ficha_url = ?, renacyt_formaciones_academicas_json = ? WHERE id_docente = ?"
    )
    .bind(&docente.renacyt_codigo_registro)
    .bind(&docente.renacyt_id_investigador)
    .bind(&docente.renacyt_nivel)
    .bind(&docente.renacyt_grupo)
    .bind(&docente.renacyt_condicion)
    .bind(docente.renacyt_fecha_informe_calificacion)
    .bind(docente.renacyt_fecha_registro)
    .bind(docente.renacyt_fecha_ultima_revision)
    .bind(&docente.renacyt_orcid)
    .bind(&docente.renacyt_scopus_author_id)
    .bind(docente.renacyt_fecha_ultima_sincronizacion)
    .bind(&docente.renacyt_ficha_url)
    .bind(&docente.renacyt_formaciones_academicas_json)
    .bind(&docente.id_docente)
    .execute(pool)
    .await?;

    Ok(())
}

// NEW: Get all docentes with project details (for detailed reporting)
pub async fn get_all_docentes_con_proyectos(pool: &SqlitePool) -> Result<Vec<DocenteDetalle>, AppError> {
    let docentes = query_as::<_, DocenteDetalle>(
        r#"
        SELECT
            d.id_docente,
            d.dni,
            d.nombres_apellidos,
            d.nombres,
            d.apellido_paterno,
            d.apellido_materno,
            g.nombre as "grado",
            COALESCE(COUNT(p.id_proyecto), 0) as "cantidad_proyectos",
            GROUP_CONCAT(p.titulo_proyecto, ' | ') as "proyectos",
            d.activo as "activo",
            d.renacyt_codigo_registro as "renacyt_codigo_registro",
            d.renacyt_id_investigador as "renacyt_id_investigador",
            d.renacyt_nivel as "renacyt_nivel",
            d.renacyt_grupo as "renacyt_grupo",
            d.renacyt_condicion as "renacyt_condicion",
            d.renacyt_fecha_informe_calificacion as "renacyt_fecha_informe_calificacion",
            d.renacyt_fecha_registro as "renacyt_fecha_registro",
            d.renacyt_fecha_ultima_revision as "renacyt_fecha_ultima_revision",
            d.renacyt_orcid as "renacyt_orcid",
            d.renacyt_scopus_author_id as "renacyt_scopus_author_id",
            d.renacyt_fecha_ultima_sincronizacion as "renacyt_fecha_ultima_sincronizacion",
            d.renacyt_ficha_url as "renacyt_ficha_url",
            d.renacyt_formaciones_academicas_json as "renacyt_formaciones_academicas_json"
        FROM docente d
        INNER JOIN grado_academico g ON d.id_grado = g.id_grado
        LEFT JOIN participacion pa ON d.id_docente = pa.id_docente
        LEFT JOIN proyecto p ON pa.id_proyecto = p.id_proyecto AND p.activo = 1
        GROUP BY d.id_docente
        ORDER BY d.nombres_apellidos ASC
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(docentes)
}

pub async fn get_docente_detalle_by_id(pool: &SqlitePool, id_docente: &str) -> Result<DocenteDetalle, AppError> {
    query_as::<_, DocenteDetalle>(
        r#"
        SELECT
            d.id_docente,
            d.dni,
            d.nombres_apellidos,
            d.nombres,
            d.apellido_paterno,
            d.apellido_materno,
            g.nombre as "grado",
            COALESCE(COUNT(p.id_proyecto), 0) as "cantidad_proyectos",
            GROUP_CONCAT(p.titulo_proyecto, ' | ') as "proyectos",
            d.activo as "activo",
            d.renacyt_codigo_registro as "renacyt_codigo_registro",
            d.renacyt_id_investigador as "renacyt_id_investigador",
            d.renacyt_nivel as "renacyt_nivel",
            d.renacyt_grupo as "renacyt_grupo",
            d.renacyt_condicion as "renacyt_condicion",
            d.renacyt_fecha_informe_calificacion as "renacyt_fecha_informe_calificacion",
            d.renacyt_fecha_registro as "renacyt_fecha_registro",
            d.renacyt_fecha_ultima_revision as "renacyt_fecha_ultima_revision",
            d.renacyt_orcid as "renacyt_orcid",
            d.renacyt_scopus_author_id as "renacyt_scopus_author_id",
            d.renacyt_fecha_ultima_sincronizacion as "renacyt_fecha_ultima_sincronizacion",
            d.renacyt_ficha_url as "renacyt_ficha_url",
            d.renacyt_formaciones_academicas_json as "renacyt_formaciones_academicas_json"
        FROM docente d
        INNER JOIN grado_academico g ON d.id_grado = g.id_grado
        LEFT JOIN participacion pa ON d.id_docente = pa.id_docente
        LEFT JOIN proyecto p ON pa.id_proyecto = p.id_proyecto AND p.activo = 1
        WHERE d.id_docente = ?
        GROUP BY d.id_docente
        "#
    )
    .bind(id_docente)
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}

pub async fn delete_docente(pool: &SqlitePool, id_docente: &str) -> Result<EliminarDocenteResultado, AppError> {
    let participaciones: (i64,) = query_as("SELECT COUNT(*) FROM participacion WHERE id_docente = ?")
        .bind(id_docente)
        .fetch_one(pool)
        .await?;

    query("UPDATE docente SET activo = 0 WHERE id_docente = ?")
        .bind(id_docente)
        .execute(pool)
        .await?;

    Ok(docente_service::build_delete_result(participaciones.0 > 0))
}

pub async fn reactivar_docente(pool: &SqlitePool, id_docente: &str) -> Result<Docente, AppError> {
    query("UPDATE docente SET activo = 1 WHERE id_docente = ?")
        .bind(id_docente)
        .execute(pool)
        .await?;

    let docente = query_as::<_, Docente>(
        "SELECT id_docente, dni, id_grado, nombres_apellidos, nombres, apellido_paterno, apellido_materno, activo, renacyt_codigo_registro, renacyt_id_investigador, renacyt_nivel, renacyt_grupo, renacyt_condicion, renacyt_fecha_informe_calificacion, renacyt_fecha_registro, renacyt_fecha_ultima_revision, renacyt_orcid, renacyt_scopus_author_id, renacyt_fecha_ultima_sincronizacion, renacyt_ficha_url, renacyt_formaciones_academicas_json FROM docente WHERE id_docente = ?"
    )
    .bind(id_docente)
    .fetch_one(pool)
    .await?;

    Ok(docente)
}