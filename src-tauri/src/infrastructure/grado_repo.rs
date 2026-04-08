use sqlx::{query, query_as, SqlitePool};
use crate::domain::grado::{GradoAcademico, CreateGradoRequest, EliminarGradoResultado};
use crate::error::AppError;

pub async fn create_grado(pool: &SqlitePool, request: CreateGradoRequest) -> Result<GradoAcademico, AppError> {
    let grado = GradoAcademico::new(request);
    query("INSERT INTO grado_academico (id_grado, nombre, descripcion, activo) VALUES (?, ?, ?, ?)")
        .bind(&grado.id_grado)
        .bind(&grado.nombre)
        .bind(&grado.descripcion)
        .bind(grado.activo)
        .execute(pool)
        .await?;
    Ok(grado)
}

pub async fn get_all_grados(pool: &SqlitePool) -> Result<Vec<GradoAcademico>, AppError> {
    let grados = query_as::<_, GradoAcademico>("SELECT id_grado, nombre, descripcion, activo FROM grado_academico ORDER BY nombre")
        .fetch_all(pool)
        .await?;
    Ok(grados)
}

pub async fn get_grado_by_id(pool: &SqlitePool, id_grado: &str) -> Result<GradoAcademico, AppError> {
    let grado = query_as::<_, GradoAcademico>("SELECT id_grado, nombre, descripcion, activo FROM grado_academico WHERE id_grado = ?")
        .bind(id_grado)
        .fetch_one(pool)
        .await?;
    Ok(grado)
}

pub async fn update_grado(pool: &SqlitePool, id_grado: &str, request: CreateGradoRequest) -> Result<GradoAcademico, AppError> {
    query("UPDATE grado_academico SET nombre = ?, descripcion = ? WHERE id_grado = ?")
        .bind(&request.nombre)
        .bind(&request.descripcion)
        .bind(id_grado)
        .execute(pool)
        .await?;
    get_grado_by_id(pool, id_grado).await
}

pub async fn delete_grado(pool: &SqlitePool, id_grado: &str) -> Result<EliminarGradoResultado, AppError> {
    let docentes_relacionados: (i64,) = query_as("SELECT COUNT(*) FROM docente WHERE id_grado = ?")
        .bind(id_grado)
        .fetch_one(pool)
        .await?;

    if docentes_relacionados.0 > 0 {
        query("UPDATE grado_academico SET activo = 0 WHERE id_grado = ?")
            .bind(id_grado)
            .execute(pool)
            .await?;
        return Ok(EliminarGradoResultado {
            accion: "desactivado".to_string(),
            mensaje: "El grado está relacionado con docentes. Se desactivó en lugar de eliminarse.".to_string(),
        });
    }

    query("DELETE FROM grado_academico WHERE id_grado = ?")
        .bind(id_grado)
        .execute(pool)
        .await?;

    Ok(EliminarGradoResultado {
        accion: "eliminado".to_string(),
        mensaje: "Grado eliminado correctamente.".to_string(),
    })
}

pub async fn reactivar_grado(pool: &SqlitePool, id_grado: &str) -> Result<GradoAcademico, AppError> {
    query("UPDATE grado_academico SET activo = 1 WHERE id_grado = ?")
        .bind(id_grado)
        .execute(pool)
        .await?;
    get_grado_by_id(pool, id_grado).await
}