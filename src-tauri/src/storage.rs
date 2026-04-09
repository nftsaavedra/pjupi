use crate::config::DatabaseBackend;
use crate::domain::docente::{CreateDocenteRequest, Docente, DocenteDetalle, EliminarDocenteResultado};
use crate::domain::estadisticas::{DocenteProyectosCount, ExportData, KpisDashboard};
use crate::domain::grado::{CreateGradoRequest, EliminarGradoResultado, GradoAcademico};
use crate::domain::proyecto::{
    CreateProyectoConParticipantesRequest,
    EliminarProyectoResultado,
    ExportDataConProjectos,
    Proyecto,
    ProyectoDetalle,
};
use crate::domain::usuario::{AuthStatus, BootstrapUsuarioRequest, CreateUsuarioRequest, LoginUsuarioRequest, UpdateUsuarioRequest, Usuario};
use crate::error::AppError;
use crate::infrastructure::{docente_repo, grado_repo, mongo_repo, proyecto_repo, usuario_repo};
use crate::state::AppState;

pub async fn get_all_grados(state: &AppState) -> Result<Vec<GradoAcademico>, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => grado_repo::get_all_grados(state.sqlite_pool()?).await,
        DatabaseBackend::MongoDb => mongo_repo::get_all_grados(state.mongo_db()?).await,
    }
}

pub async fn crear_grado(state: &AppState, request: CreateGradoRequest) -> Result<GradoAcademico, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => grado_repo::create_grado(state.sqlite_pool()?, request).await,
        DatabaseBackend::MongoDb => mongo_repo::create_grado(state.mongo_db()?, request).await,
    }
}

pub async fn actualizar_grado(state: &AppState, id_grado: &str, request: CreateGradoRequest) -> Result<GradoAcademico, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => grado_repo::update_grado(state.sqlite_pool()?, id_grado, request).await,
        DatabaseBackend::MongoDb => mongo_repo::update_grado(state.mongo_db()?, id_grado, request).await,
    }
}

pub async fn eliminar_grado(state: &AppState, id_grado: &str) -> Result<EliminarGradoResultado, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => grado_repo::delete_grado(state.sqlite_pool()?, id_grado).await,
        DatabaseBackend::MongoDb => mongo_repo::delete_grado(state.mongo_db()?, id_grado).await,
    }
}

pub async fn reactivar_grado(state: &AppState, id_grado: &str) -> Result<GradoAcademico, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => grado_repo::reactivar_grado(state.sqlite_pool()?, id_grado).await,
        DatabaseBackend::MongoDb => mongo_repo::reactivar_grado(state.mongo_db()?, id_grado).await,
    }
}

pub async fn crear_docente(state: &AppState, request: CreateDocenteRequest) -> Result<Docente, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => docente_repo::create_docente(state.sqlite_pool()?, request).await,
        DatabaseBackend::MongoDb => mongo_repo::create_docente(state.mongo_db()?, request).await,
    }
}

pub async fn get_all_docentes(state: &AppState) -> Result<Vec<Docente>, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => docente_repo::get_all_docentes(state.sqlite_pool()?).await,
        DatabaseBackend::MongoDb => mongo_repo::get_all_docentes(state.mongo_db()?).await,
    }
}

pub async fn buscar_docente_por_dni(state: &AppState, dni: &str) -> Result<Option<Docente>, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => docente_repo::get_docente_by_dni(state.sqlite_pool()?, dni).await,
        DatabaseBackend::MongoDb => mongo_repo::get_docente_by_dni(state.mongo_db()?, dni).await,
    }
}

pub async fn get_all_docentes_con_proyectos(state: &AppState) -> Result<Vec<DocenteDetalle>, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => docente_repo::get_all_docentes_con_proyectos(state.sqlite_pool()?).await,
        DatabaseBackend::MongoDb => mongo_repo::get_all_docentes_con_proyectos(state.mongo_db()?).await,
    }
}

pub async fn eliminar_docente(state: &AppState, id_docente: &str) -> Result<EliminarDocenteResultado, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => docente_repo::delete_docente(state.sqlite_pool()?, id_docente).await,
        DatabaseBackend::MongoDb => mongo_repo::delete_docente(state.mongo_db()?, id_docente).await,
    }
}

pub async fn reactivar_docente(state: &AppState, id_docente: &str) -> Result<Docente, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => docente_repo::reactivar_docente(state.sqlite_pool()?, id_docente).await,
        DatabaseBackend::MongoDb => mongo_repo::reactivar_docente(state.mongo_db()?, id_docente).await,
    }
}

pub async fn crear_proyecto_con_participantes(state: &AppState, request: CreateProyectoConParticipantesRequest) -> Result<Proyecto, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::create_proyecto_con_participantes(state.sqlite_pool()?, request).await,
        DatabaseBackend::MongoDb => mongo_repo::create_proyecto_con_participantes(state.mongo_db()?, request).await,
    }
}

pub async fn buscar_proyectos_por_docente(state: &AppState, id_docente: &str) -> Result<Vec<Proyecto>, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::buscar_proyectos_por_docente(state.sqlite_pool()?, id_docente).await,
        DatabaseBackend::MongoDb => mongo_repo::buscar_proyectos_por_docente(state.mongo_db()?, id_docente).await,
    }
}

pub async fn get_all_proyectos_detalle(state: &AppState) -> Result<Vec<ProyectoDetalle>, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::get_all_proyectos_detalle(state.sqlite_pool()?).await,
        DatabaseBackend::MongoDb => mongo_repo::get_all_proyectos_detalle(state.mongo_db()?).await,
    }
}

pub async fn eliminar_relacion_proyecto_docente(state: &AppState, id_proyecto: &str, id_docente: &str) -> Result<(), AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::eliminar_relacion_proyecto_docente(state.sqlite_pool()?, id_proyecto, id_docente).await,
        DatabaseBackend::MongoDb => mongo_repo::eliminar_relacion_proyecto_docente(state.mongo_db()?, id_proyecto, id_docente).await,
    }
}

pub async fn eliminar_relaciones_proyecto(state: &AppState, id_proyecto: &str) -> Result<(), AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::eliminar_relaciones_proyecto(state.sqlite_pool()?, id_proyecto).await,
        DatabaseBackend::MongoDb => mongo_repo::eliminar_relaciones_proyecto(state.mongo_db()?, id_proyecto).await,
    }
}

pub async fn eliminar_proyecto(state: &AppState, id_proyecto: &str) -> Result<EliminarProyectoResultado, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::eliminar_proyecto(state.sqlite_pool()?, id_proyecto).await,
        DatabaseBackend::MongoDb => mongo_repo::eliminar_proyecto(state.mongo_db()?, id_proyecto).await,
    }
}

pub async fn reactivar_proyecto(state: &AppState, id_proyecto: &str) -> Result<Proyecto, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::reactivar_proyecto(state.sqlite_pool()?, id_proyecto).await,
        DatabaseBackend::MongoDb => mongo_repo::reactivar_proyecto(state.mongo_db()?, id_proyecto).await,
    }
}

pub async fn get_estadisticas_proyectos_x_docente(state: &AppState) -> Result<Vec<DocenteProyectosCount>, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::get_estadisticas_proyectos_x_docente(state.sqlite_pool()?).await,
        DatabaseBackend::MongoDb => mongo_repo::get_estadisticas_proyectos_x_docente(state.mongo_db()?).await,
    }
}

pub async fn get_kpis_dashboard(state: &AppState) -> Result<KpisDashboard, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::get_kpis_dashboard(state.sqlite_pool()?).await,
        DatabaseBackend::MongoDb => mongo_repo::get_kpis_dashboard(state.mongo_db()?).await,
    }
}

pub async fn get_data_exportacion_plana(state: &AppState) -> Result<Vec<ExportData>, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::get_data_exportacion_plana(state.sqlite_pool()?).await,
        DatabaseBackend::MongoDb => mongo_repo::get_data_exportacion_plana(state.mongo_db()?).await,
    }
}

pub async fn get_data_exportacion_agrupada_docente(state: &AppState) -> Result<Vec<ExportDataConProjectos>, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::get_data_exportacion_agrupada_docente(state.sqlite_pool()?).await,
        DatabaseBackend::MongoDb => mongo_repo::get_data_exportacion_agrupada_docente(state.mongo_db()?).await,
    }
}

pub async fn crear_usuario(state: &AppState, request: CreateUsuarioRequest) -> Result<Usuario, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => usuario_repo::create_usuario(state.sqlite_pool()?, request).await,
        DatabaseBackend::MongoDb => mongo_repo::create_usuario(state.mongo_db()?, request).await,
    }
}

pub async fn get_auth_status(state: &AppState) -> Result<AuthStatus, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => usuario_repo::get_auth_status(state.sqlite_pool()?).await,
        DatabaseBackend::MongoDb => mongo_repo::get_auth_status(state.mongo_db()?).await,
    }
}

pub async fn registrar_primer_usuario(state: &AppState, request: BootstrapUsuarioRequest) -> Result<Usuario, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => usuario_repo::bootstrap_admin(state.sqlite_pool()?, request).await,
        DatabaseBackend::MongoDb => mongo_repo::bootstrap_admin(state.mongo_db()?, request).await,
    }
}

pub async fn login_usuario(state: &AppState, request: LoginUsuarioRequest) -> Result<Usuario, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => usuario_repo::login_usuario(state.sqlite_pool()?, request).await,
        DatabaseBackend::MongoDb => mongo_repo::login_usuario(state.mongo_db()?, request).await,
    }
}

pub async fn get_all_usuarios(state: &AppState) -> Result<Vec<Usuario>, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => usuario_repo::get_all_usuarios(state.sqlite_pool()?).await,
        DatabaseBackend::MongoDb => mongo_repo::get_all_usuarios(state.mongo_db()?).await,
    }
}

pub async fn actualizar_usuario(state: &AppState, id_usuario: &str, request: UpdateUsuarioRequest) -> Result<Usuario, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => usuario_repo::update_usuario(state.sqlite_pool()?, id_usuario, request).await,
        DatabaseBackend::MongoDb => mongo_repo::update_usuario(state.mongo_db()?, id_usuario, request).await,
    }
}

pub async fn desactivar_usuario(state: &AppState, id_usuario: &str) -> Result<Usuario, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => usuario_repo::desactivar_usuario(state.sqlite_pool()?, id_usuario).await,
        DatabaseBackend::MongoDb => mongo_repo::desactivar_usuario(state.mongo_db()?, id_usuario).await,
    }
}

pub async fn reactivar_usuario(state: &AppState, id_usuario: &str) -> Result<Usuario, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => usuario_repo::reactivar_usuario(state.sqlite_pool()?, id_usuario).await,
        DatabaseBackend::MongoDb => mongo_repo::reactivar_usuario(state.mongo_db()?, id_usuario).await,
    }
}