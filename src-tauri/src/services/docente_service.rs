use crate::domain::docente::{CreateDocenteRequest, Docente, DocenteDetalle, EliminarDocenteResultado};
use crate::error::AppError;
use crate::infrastructure::mongo_repo;
use crate::state::AppState;

pub fn build_delete_result(has_related_projects: bool) -> EliminarDocenteResultado {
    if has_related_projects {
        return EliminarDocenteResultado {
            accion: "desactivado".to_string(),
            mensaje: "Docente desactivado. Mantiene trazabilidad porque tiene proyectos relacionados.".to_string(),
        };
    }

    EliminarDocenteResultado {
        accion: "desactivado".to_string(),
        mensaje: "Docente desactivado correctamente.".to_string(),
    }
}

pub async fn create(state: &AppState, request: CreateDocenteRequest) -> Result<Docente, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::create_docente(db, request).await
}

pub async fn get_all(state: &AppState) -> Result<Vec<Docente>, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_all_docentes(db).await
}

pub async fn find_by_dni(state: &AppState, dni: &str) -> Result<Option<Docente>, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_docente_by_dni(db, dni).await
}

pub async fn get_all_detalle(state: &AppState) -> Result<Vec<DocenteDetalle>, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_all_docentes_con_proyectos(db).await
}

pub async fn delete(state: &AppState, id_docente: &str) -> Result<EliminarDocenteResultado, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::delete_docente(db, id_docente).await
}

pub async fn reactivate(state: &AppState, id_docente: &str) -> Result<Docente, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::reactivar_docente(db, id_docente).await
}

pub async fn get_by_id(state: &AppState, id_docente: &str) -> Result<Docente, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_docente_by_id(db, id_docente).await
}

pub async fn update_renacyt(state: &AppState, docente: &Docente) -> Result<(), AppError> {
    let db = state.mongo_db()?;
    mongo_repo::update_docente_renacyt(db, docente).await
}

pub async fn get_detalle_by_id(state: &AppState, id_docente: &str) -> Result<DocenteDetalle, AppError> {
    let db = state.mongo_db()?;
    mongo_repo::get_docente_detalle_by_id(db, id_docente).await
}
