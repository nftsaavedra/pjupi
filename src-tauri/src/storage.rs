use crate::config::DatabaseBackend;
use crate::domain::docente::{CreateDocenteRequest, Docente, DocenteDetalle, EliminarDocenteResultado, RefreshDocenteRenacytFormacionResultado};
use crate::domain::estadisticas::{DocenteProyectosCount, ExportData, KpisDashboard};
use crate::domain::grado::{CreateGradoRequest, EliminarGradoResultado, GradoAcademico};
use crate::domain::proyecto::{
    CreateProyectoConParticipantesRequest,
    EliminarProyectoResultado,
    ExportDataConProjectos,
    Proyecto,
    ProyectoDetalle,
    UpdateProyectoConParticipantesRequest,
};
use crate::domain::usuario::{AuthStatus, BootstrapUsuarioRequest, CreateUsuarioRequest, LoginUsuarioRequest, UpdateUsuarioRequest, Usuario};
use crate::error::AppError;
use crate::infrastructure::{docente_repo, grado_repo, mongo_repo, proyecto_repo, usuario_repo};
use crate::state::AppState;

enum AppPermission {
    DashboardView,
    DocentesView,
    DocentesManage,
    ProyectosView,
    ProyectosManage,
    ReportesView,
    ReportesExport,
    GradosRead,
    GradosManage,
}

fn role_has_permission(role: &str, permission: &AppPermission) -> bool {
    match role.trim() {
        "admin" => true,
        "operador" => matches!(
            permission,
            AppPermission::DashboardView
                | AppPermission::DocentesView
                | AppPermission::DocentesManage
                | AppPermission::ProyectosView
                | AppPermission::ProyectosManage
                | AppPermission::ReportesView
                | AppPermission::ReportesExport
                | AppPermission::GradosRead
        ),
        "consulta" => matches!(
            permission,
            AppPermission::DashboardView
                | AppPermission::DocentesView
                | AppPermission::ProyectosView
                | AppPermission::ReportesView
        ),
        _ => false,
    }
}

async fn require_permission(state: &AppState, window_label: &str, permission: AppPermission) -> Result<Usuario, AppError> {
    let actor = get_session_actor_user(state, window_label).await?;

    if !role_has_permission(&actor.rol, &permission) {
        return Err(AppError::InternalError("No tiene permisos para ejecutar esta operación.".to_string()));
    }

    Ok(actor)
}

pub async fn require_docentes_manage_permission(state: &AppState, window_label: &str) -> Result<Usuario, AppError> {
    require_permission(state, window_label, AppPermission::DocentesManage).await
}

async fn get_user_by_id(state: &AppState, user_id: &str) -> Result<Usuario, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => Ok(usuario_repo::get_usuario_by_id(state.sqlite_pool()?, user_id).await?.public_view()),
        DatabaseBackend::MongoDb => Ok(mongo_repo::get_usuario_by_id(state.mongo_db()?, user_id).await?.public_view()),
    }
}

async fn get_session_actor_user(state: &AppState, window_label: &str) -> Result<Usuario, AppError> {
    let actor_user_id = state
        .get_current_session_user_id(window_label)
        .await
        .ok_or_else(|| AppError::InternalError("No hay una sesión activa. Inicie sesión para continuar.".to_string()))?;

    let actor = match get_user_by_id(state, &actor_user_id).await {
        Ok(actor) => actor,
        Err(AppError::NotFound(_)) => {
            state.clear_current_session(window_label).await;
            return Err(AppError::InternalError("La sesión actual ya no es válida. Inicie sesión nuevamente.".to_string()));
        }
        Err(error) => return Err(error),
    };

    if actor.activo == 0 {
        state.clear_current_session(window_label).await;
        return Err(AppError::InternalError("La sesión actual pertenece a un usuario inactivo. Inicie sesión nuevamente.".to_string()));
    }

    state.touch_current_session(window_label).await;
    Ok(actor)
}

pub async fn get_all_grados(state: &AppState, window_label: &str) -> Result<Vec<GradoAcademico>, AppError> {
    require_permission(state, window_label, AppPermission::GradosRead).await?;
    match state.backend {
        DatabaseBackend::Sqlite => grado_repo::get_all_grados(state.sqlite_pool()?).await,
        DatabaseBackend::MongoDb => mongo_repo::get_all_grados(state.mongo_db()?).await,
    }
}

pub async fn crear_grado(state: &AppState, window_label: &str, request: CreateGradoRequest) -> Result<GradoAcademico, AppError> {
    require_permission(state, window_label, AppPermission::GradosManage).await?;
    match state.backend {
        DatabaseBackend::Sqlite => grado_repo::create_grado(state.sqlite_pool()?, request).await,
        DatabaseBackend::MongoDb => mongo_repo::create_grado(state.mongo_db()?, request).await,
    }
}

pub async fn actualizar_grado(state: &AppState, window_label: &str, id_grado: &str, request: CreateGradoRequest) -> Result<GradoAcademico, AppError> {
    require_permission(state, window_label, AppPermission::GradosManage).await?;
    match state.backend {
        DatabaseBackend::Sqlite => grado_repo::update_grado(state.sqlite_pool()?, id_grado, request).await,
        DatabaseBackend::MongoDb => mongo_repo::update_grado(state.mongo_db()?, id_grado, request).await,
    }
}

pub async fn eliminar_grado(state: &AppState, window_label: &str, id_grado: &str) -> Result<EliminarGradoResultado, AppError> {
    require_permission(state, window_label, AppPermission::GradosManage).await?;
    match state.backend {
        DatabaseBackend::Sqlite => grado_repo::delete_grado(state.sqlite_pool()?, id_grado).await,
        DatabaseBackend::MongoDb => mongo_repo::delete_grado(state.mongo_db()?, id_grado).await,
    }
}

pub async fn reactivar_grado(state: &AppState, window_label: &str, id_grado: &str) -> Result<GradoAcademico, AppError> {
    require_permission(state, window_label, AppPermission::GradosManage).await?;
    match state.backend {
        DatabaseBackend::Sqlite => grado_repo::reactivar_grado(state.sqlite_pool()?, id_grado).await,
        DatabaseBackend::MongoDb => mongo_repo::reactivar_grado(state.mongo_db()?, id_grado).await,
    }
}

pub async fn crear_docente(state: &AppState, window_label: &str, request: CreateDocenteRequest) -> Result<Docente, AppError> {
    require_permission(state, window_label, AppPermission::DocentesManage).await?;
    match state.backend {
        DatabaseBackend::Sqlite => docente_repo::create_docente(state.sqlite_pool()?, request).await,
        DatabaseBackend::MongoDb => mongo_repo::create_docente(state.mongo_db()?, request).await,
    }
}

pub async fn get_all_docentes(state: &AppState, window_label: &str) -> Result<Vec<Docente>, AppError> {
    require_permission(state, window_label, AppPermission::DocentesView).await?;
    match state.backend {
        DatabaseBackend::Sqlite => docente_repo::get_all_docentes(state.sqlite_pool()?).await,
        DatabaseBackend::MongoDb => mongo_repo::get_all_docentes(state.mongo_db()?).await,
    }
}

pub async fn buscar_docente_por_dni(state: &AppState, window_label: &str, dni: &str) -> Result<Option<Docente>, AppError> {
    require_permission(state, window_label, AppPermission::DocentesManage).await?;
    match state.backend {
        DatabaseBackend::Sqlite => docente_repo::get_docente_by_dni(state.sqlite_pool()?, dni).await,
        DatabaseBackend::MongoDb => mongo_repo::get_docente_by_dni(state.mongo_db()?, dni).await,
    }
}

pub async fn get_all_docentes_con_proyectos(state: &AppState, window_label: &str) -> Result<Vec<DocenteDetalle>, AppError> {
    require_permission(state, window_label, AppPermission::DocentesView).await?;
    match state.backend {
        DatabaseBackend::Sqlite => docente_repo::get_all_docentes_con_proyectos(state.sqlite_pool()?).await,
        DatabaseBackend::MongoDb => mongo_repo::get_all_docentes_con_proyectos(state.mongo_db()?).await,
    }
}

pub async fn eliminar_docente(state: &AppState, window_label: &str, id_docente: &str) -> Result<EliminarDocenteResultado, AppError> {
    require_permission(state, window_label, AppPermission::DocentesManage).await?;
    match state.backend {
        DatabaseBackend::Sqlite => docente_repo::delete_docente(state.sqlite_pool()?, id_docente).await,
        DatabaseBackend::MongoDb => mongo_repo::delete_docente(state.mongo_db()?, id_docente).await,
    }
}

pub async fn reactivar_docente(state: &AppState, window_label: &str, id_docente: &str) -> Result<Docente, AppError> {
    require_permission(state, window_label, AppPermission::DocentesManage).await?;
    match state.backend {
        DatabaseBackend::Sqlite => docente_repo::reactivar_docente(state.sqlite_pool()?, id_docente).await,
        DatabaseBackend::MongoDb => mongo_repo::reactivar_docente(state.mongo_db()?, id_docente).await,
    }
}

pub async fn refrescar_formacion_academica_renacyt_docente(
    state: &AppState,
    window_label: &str,
    id_docente: &str,
) -> Result<RefreshDocenteRenacytFormacionResultado, AppError> {
    require_permission(state, window_label, AppPermission::DocentesManage).await?;
    let mut docente = match state.backend {
        DatabaseBackend::Sqlite => docente_repo::get_docente_by_id(state.sqlite_pool()?, id_docente).await?,
        DatabaseBackend::MongoDb => mongo_repo::get_docente_by_id(state.mongo_db()?, id_docente).await?,
    };

    let codigo_o_id = docente
        .renacyt_id_investigador
        .clone()
        .or_else(|| docente.renacyt_codigo_registro.clone())
        .ok_or_else(|| AppError::ExternalServiceError("El docente no tiene un vínculo RENACYT para refrescar su formación académica.".to_string()))?;

    let tenia_formaciones = docente
        .renacyt_formaciones_academicas_json
        .as_ref()
        .is_some_and(|value| !value.trim().is_empty());

    let lookup = crate::infrastructure::renacyt_client::consultar_investigador(state.renacyt_config(), &codigo_o_id).await?;
    let actualizada = docente.apply_renacyt_refresh(lookup);

    match state.backend {
        DatabaseBackend::Sqlite => docente_repo::update_docente_renacyt(state.sqlite_pool()?, &docente).await?,
        DatabaseBackend::MongoDb => mongo_repo::update_docente_renacyt(state.mongo_db()?, &docente).await?,
    }

    let docente_detalle = match state.backend {
        DatabaseBackend::Sqlite => docente_repo::get_docente_detalle_by_id(state.sqlite_pool()?, id_docente).await?,
        DatabaseBackend::MongoDb => mongo_repo::get_docente_detalle_by_id(state.mongo_db()?, id_docente).await?,
    };

    let mensaje = if actualizada {
        "Formación académica RENACYT actualizada correctamente.".to_string()
    } else if tenia_formaciones {
        "RENACYT no devolvió nueva formación académica en esta sincronización. Se mantuvo la información registrada.".to_string()
    } else {
        "RENACYT no devolvió formación académica disponible para este docente en esta sincronización.".to_string()
    };

    Ok(RefreshDocenteRenacytFormacionResultado {
        docente: docente_detalle,
        actualizada,
        mensaje,
    })
}

pub async fn crear_proyecto_con_participantes(state: &AppState, window_label: &str, request: CreateProyectoConParticipantesRequest) -> Result<Proyecto, AppError> {
    require_permission(state, window_label, AppPermission::ProyectosManage).await?;
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::create_proyecto_con_participantes(state.sqlite_pool()?, request).await,
        DatabaseBackend::MongoDb => mongo_repo::create_proyecto_con_participantes(state.mongo_db()?, request).await,
    }
}

pub async fn update_proyecto_con_participantes(
    state: &AppState,
    window_label: &str,
    id_proyecto: &str,
    request: UpdateProyectoConParticipantesRequest,
) -> Result<Proyecto, AppError> {
    require_permission(state, window_label, AppPermission::ProyectosManage).await?;
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::update_proyecto_con_participantes(state.sqlite_pool()?, id_proyecto, request).await,
        DatabaseBackend::MongoDb => mongo_repo::update_proyecto_con_participantes(state.mongo_db()?, id_proyecto, request).await,
    }
}

pub async fn buscar_proyectos_por_docente(state: &AppState, window_label: &str, id_docente: &str) -> Result<Vec<Proyecto>, AppError> {
    require_permission(state, window_label, AppPermission::ProyectosView).await?;
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::buscar_proyectos_por_docente(state.sqlite_pool()?, id_docente).await,
        DatabaseBackend::MongoDb => mongo_repo::buscar_proyectos_por_docente(state.mongo_db()?, id_docente).await,
    }
}

pub async fn get_all_proyectos_detalle(state: &AppState, window_label: &str) -> Result<Vec<ProyectoDetalle>, AppError> {
    require_permission(state, window_label, AppPermission::ProyectosView).await?;
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::get_all_proyectos_detalle(state.sqlite_pool()?).await,
        DatabaseBackend::MongoDb => mongo_repo::get_all_proyectos_detalle(state.mongo_db()?).await,
    }
}

pub async fn eliminar_relacion_proyecto_docente(state: &AppState, window_label: &str, id_proyecto: &str, id_docente: &str) -> Result<(), AppError> {
    require_permission(state, window_label, AppPermission::ProyectosManage).await?;
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::eliminar_relacion_proyecto_docente(state.sqlite_pool()?, id_proyecto, id_docente).await,
        DatabaseBackend::MongoDb => mongo_repo::eliminar_relacion_proyecto_docente(state.mongo_db()?, id_proyecto, id_docente).await,
    }
}

pub async fn eliminar_relaciones_proyecto(state: &AppState, window_label: &str, id_proyecto: &str) -> Result<(), AppError> {
    require_permission(state, window_label, AppPermission::ProyectosManage).await?;
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::eliminar_relaciones_proyecto(state.sqlite_pool()?, id_proyecto).await,
        DatabaseBackend::MongoDb => mongo_repo::eliminar_relaciones_proyecto(state.mongo_db()?, id_proyecto).await,
    }
}

pub async fn eliminar_proyecto(state: &AppState, window_label: &str, id_proyecto: &str) -> Result<EliminarProyectoResultado, AppError> {
    require_permission(state, window_label, AppPermission::ProyectosManage).await?;
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::eliminar_proyecto(state.sqlite_pool()?, id_proyecto).await,
        DatabaseBackend::MongoDb => mongo_repo::eliminar_proyecto(state.mongo_db()?, id_proyecto).await,
    }
}

pub async fn reactivar_proyecto(state: &AppState, window_label: &str, id_proyecto: &str) -> Result<Proyecto, AppError> {
    require_permission(state, window_label, AppPermission::ProyectosManage).await?;
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::reactivar_proyecto(state.sqlite_pool()?, id_proyecto).await,
        DatabaseBackend::MongoDb => mongo_repo::reactivar_proyecto(state.mongo_db()?, id_proyecto).await,
    }
}

pub async fn get_estadisticas_proyectos_x_docente(state: &AppState, window_label: &str) -> Result<Vec<DocenteProyectosCount>, AppError> {
    require_permission(state, window_label, AppPermission::DashboardView).await?;
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::get_estadisticas_proyectos_x_docente(state.sqlite_pool()?).await,
        DatabaseBackend::MongoDb => mongo_repo::get_estadisticas_proyectos_x_docente(state.mongo_db()?).await,
    }
}

pub async fn get_kpis_dashboard(state: &AppState, window_label: &str) -> Result<KpisDashboard, AppError> {
    require_permission(state, window_label, AppPermission::DashboardView).await?;
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::get_kpis_dashboard(state.sqlite_pool()?).await,
        DatabaseBackend::MongoDb => mongo_repo::get_kpis_dashboard(state.mongo_db()?).await,
    }
}

pub async fn get_data_exportacion_plana(state: &AppState, window_label: &str) -> Result<Vec<ExportData>, AppError> {
    require_permission(state, window_label, AppPermission::ReportesExport).await?;
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::get_data_exportacion_plana(state.sqlite_pool()?).await,
        DatabaseBackend::MongoDb => mongo_repo::get_data_exportacion_plana(state.mongo_db()?).await,
    }
}

pub async fn get_data_exportacion_agrupada_docente(state: &AppState, window_label: &str) -> Result<Vec<ExportDataConProjectos>, AppError> {
    require_permission(state, window_label, AppPermission::ReportesView).await?;
    match state.backend {
        DatabaseBackend::Sqlite => proyecto_repo::get_data_exportacion_agrupada_docente(state.sqlite_pool()?).await,
        DatabaseBackend::MongoDb => mongo_repo::get_data_exportacion_agrupada_docente(state.mongo_db()?).await,
    }
}

pub async fn crear_usuario(state: &AppState, window_label: &str, request: CreateUsuarioRequest) -> Result<Usuario, AppError> {
    let actor = get_session_actor_user(state, window_label).await?;
    let usuario = match state.backend {
        DatabaseBackend::Sqlite => usuario_repo::create_usuario(state.sqlite_pool()?, &actor.id_usuario, request).await,
        DatabaseBackend::MongoDb => mongo_repo::create_usuario(state.mongo_db()?, &actor.id_usuario, request).await,
    }?;

    crate::audit::write_user_audit(&actor, "usuario.create", &usuario, format!("rol={}", usuario.rol));
    Ok(usuario)
}

pub async fn get_auth_status(state: &AppState) -> Result<AuthStatus, AppError> {
    match state.backend {
        DatabaseBackend::Sqlite => usuario_repo::get_auth_status(state.sqlite_pool()?).await,
        DatabaseBackend::MongoDb => mongo_repo::get_auth_status(state.mongo_db()?).await,
    }
}

pub async fn registrar_primer_usuario(state: &AppState, window_label: &str, request: BootstrapUsuarioRequest) -> Result<Usuario, AppError> {
    let usuario = match state.backend {
        DatabaseBackend::Sqlite => usuario_repo::bootstrap_admin(state.sqlite_pool()?, request).await,
        DatabaseBackend::MongoDb => mongo_repo::bootstrap_admin(state.mongo_db()?, request).await,
    }?;

    state.set_current_session(window_label, usuario.id_usuario.clone()).await;
    Ok(usuario)
}

pub async fn login_usuario(state: &AppState, window_label: &str, request: LoginUsuarioRequest) -> Result<Usuario, AppError> {
    let usuario = match state.backend {
        DatabaseBackend::Sqlite => usuario_repo::login_usuario(state.sqlite_pool()?, request).await,
        DatabaseBackend::MongoDb => mongo_repo::login_usuario(state.mongo_db()?, request).await,
    }?;

    state.set_current_session(window_label, usuario.id_usuario.clone()).await;
    Ok(usuario)
}

pub async fn get_current_session(state: &AppState, window_label: &str) -> Result<Option<Usuario>, AppError> {
    let Some(actor_user_id) = state.get_current_session_user_id(window_label).await else {
        return Ok(None);
    };

    let actor = match get_user_by_id(state, &actor_user_id).await {
        Ok(actor) if actor.activo == 1 => actor,
        Ok(_) | Err(AppError::NotFound(_)) => {
            state.clear_current_session(window_label).await;
            return Ok(None);
        }
        Err(error) => return Err(error),
    };

    state.touch_current_session(window_label).await;
    Ok(Some(actor))
}

pub async fn logout_usuario(state: &AppState, window_label: &str) -> Result<(), AppError> {
    state.clear_current_session(window_label).await;
    Ok(())
}

pub async fn get_all_usuarios(state: &AppState, window_label: &str) -> Result<Vec<Usuario>, AppError> {
    let actor = get_session_actor_user(state, window_label).await?;
    match state.backend {
        DatabaseBackend::Sqlite => usuario_repo::get_all_usuarios(state.sqlite_pool()?, &actor.id_usuario).await,
        DatabaseBackend::MongoDb => mongo_repo::get_all_usuarios(state.mongo_db()?, &actor.id_usuario).await,
    }
}

pub async fn actualizar_usuario(state: &AppState, window_label: &str, id_usuario: &str, request: UpdateUsuarioRequest) -> Result<Usuario, AppError> {
    let actor = get_session_actor_user(state, window_label).await?;
    let previous_user = get_user_by_id(state, id_usuario).await?;
    let usuario = match state.backend {
        DatabaseBackend::Sqlite => usuario_repo::update_usuario(state.sqlite_pool()?, &actor.id_usuario, id_usuario, request).await,
        DatabaseBackend::MongoDb => mongo_repo::update_usuario(state.mongo_db()?, &actor.id_usuario, id_usuario, request).await,
    }?;

    crate::audit::write_user_audit(
        &actor,
        "usuario.update",
        &usuario,
        format!(
            "username:{}->{}; rol:{}->{}; activo:{}",
            previous_user.username,
            usuario.username,
            previous_user.rol,
            usuario.rol,
            usuario.activo,
        ),
    );
    Ok(usuario)
}

pub async fn desactivar_usuario(state: &AppState, window_label: &str, id_usuario: &str) -> Result<Usuario, AppError> {
    let actor = get_session_actor_user(state, window_label).await?;
    let usuario = match state.backend {
        DatabaseBackend::Sqlite => usuario_repo::desactivar_usuario(state.sqlite_pool()?, &actor.id_usuario, id_usuario).await,
        DatabaseBackend::MongoDb => mongo_repo::desactivar_usuario(state.mongo_db()?, &actor.id_usuario, id_usuario).await,
    }?;

    crate::audit::write_user_audit(&actor, "usuario.deactivate", &usuario, "activo=0".to_string());
    Ok(usuario)
}

pub async fn reactivar_usuario(state: &AppState, window_label: &str, id_usuario: &str) -> Result<Usuario, AppError> {
    let actor = get_session_actor_user(state, window_label).await?;
    let usuario = match state.backend {
        DatabaseBackend::Sqlite => usuario_repo::reactivar_usuario(state.sqlite_pool()?, &actor.id_usuario, id_usuario).await,
        DatabaseBackend::MongoDb => mongo_repo::reactivar_usuario(state.mongo_db()?, &actor.id_usuario, id_usuario).await,
    }?;

    crate::audit::write_user_audit(&actor, "usuario.reactivate", &usuario, "activo=1".to_string());
    Ok(usuario)
}