use sqlx::{query, query_as, SqlitePool};
use crate::domain::docente::{Docente, CreateDocenteRequest, DocenteDetalle, EliminarDocenteResultado};
use crate::error::AppError;

pub async fn create_docente(pool: &SqlitePool, request: CreateDocenteRequest) -> Result<Docente, AppError> {
    let docente = Docente::new(request);
    query("INSERT INTO docente (id_docente, dni, id_grado, nombres_apellidos, activo) VALUES (?, ?, ?, ?, ?)")
        .bind(&docente.id_docente)
        .bind(&docente.dni)
        .bind(&docente.id_grado)
        .bind(&docente.nombres_apellidos)
        .bind(docente.activo)
        .execute(pool)
        .await?;
    Ok(docente)
}

pub async fn get_all_docentes(pool: &SqlitePool) -> Result<Vec<Docente>, AppError> {
    let docentes = query_as::<_, Docente>(
        "SELECT id_docente, dni, id_grado, nombres_apellidos, activo FROM docente WHERE activo = 1"
    )
        .fetch_all(pool)
        .await?;
    Ok(docentes)
}

// NEW: Get all docentes with project details (for detailed reporting)
pub async fn get_all_docentes_con_proyectos(pool: &SqlitePool) -> Result<Vec<DocenteDetalle>, AppError> {
    let docentes = query_as::<_, DocenteDetalle>(
        r#"
        SELECT
            d.id_docente,
            d.dni,
            d.nombres_apellidos,
            g.nombre as "grado",
            COALESCE(COUNT(p.id_proyecto), 0) as "cantidad_proyectos",
            GROUP_CONCAT(p.titulo_proyecto, ', ') as "proyectos",
            d.activo as "activo"
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

pub async fn delete_docente(pool: &SqlitePool, id_docente: &str) -> Result<EliminarDocenteResultado, AppError> {
    let participaciones: (i64,) = query_as("SELECT COUNT(*) FROM participacion WHERE id_docente = ?")
        .bind(id_docente)
        .fetch_one(pool)
        .await?;

    query("UPDATE docente SET activo = 0 WHERE id_docente = ?")
        .bind(id_docente)
        .execute(pool)
        .await?;

    if participaciones.0 > 0 {
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

pub async fn reactivar_docente(pool: &SqlitePool, id_docente: &str) -> Result<Docente, AppError> {
    query("UPDATE docente SET activo = 1 WHERE id_docente = ?")
        .bind(id_docente)
        .execute(pool)
        .await?;

    let docente = query_as::<_, Docente>(
        "SELECT id_docente, dni, id_grado, nombres_apellidos, activo FROM docente WHERE id_docente = ?"
    )
    .bind(id_docente)
    .fetch_one(pool)
    .await?;

    Ok(docente)
}