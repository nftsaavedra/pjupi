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
use crate::services::{docente_service, grado_service, proyecto_service, usuario_service};
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
    usuario_service::get_by_id_public(state, user_id).await
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
    grado_service::get_all(state).await
}

pub async fn crear_grado(state: &AppState, window_label: &str, request: CreateGradoRequest) -> Result<GradoAcademico, AppError> {
    require_permission(state, window_label, AppPermission::GradosManage).await?;
    grado_service::create(state, request).await
}

pub async fn actualizar_grado(state: &AppState, window_label: &str, id_grado: &str, request: CreateGradoRequest) -> Result<GradoAcademico, AppError> {
    require_permission(state, window_label, AppPermission::GradosManage).await?;
    grado_service::update(state, id_grado, request).await
}

pub async fn eliminar_grado(state: &AppState, window_label: &str, id_grado: &str) -> Result<EliminarGradoResultado, AppError> {
    require_permission(state, window_label, AppPermission::GradosManage).await?;
    grado_service::delete(state, id_grado).await
}

pub async fn reactivar_grado(state: &AppState, window_label: &str, id_grado: &str) -> Result<GradoAcademico, AppError> {
    require_permission(state, window_label, AppPermission::GradosManage).await?;
    grado_service::reactivate(state, id_grado).await
}

pub async fn crear_docente(state: &AppState, window_label: &str, request: CreateDocenteRequest) -> Result<Docente, AppError> {
    require_permission(state, window_label, AppPermission::DocentesManage).await?;
    docente_service::create(state, request).await
}

pub async fn get_all_docentes(state: &AppState, window_label: &str) -> Result<Vec<Docente>, AppError> {
    require_permission(state, window_label, AppPermission::DocentesView).await?;
    docente_service::get_all(state).await
}

pub async fn buscar_docente_por_dni(state: &AppState, window_label: &str, dni: &str) -> Result<Option<Docente>, AppError> {
    require_permission(state, window_label, AppPermission::DocentesManage).await?;
    docente_service::find_by_dni(state, dni).await
}

pub async fn get_all_docentes_con_proyectos(state: &AppState, window_label: &str) -> Result<Vec<DocenteDetalle>, AppError> {
    require_permission(state, window_label, AppPermission::DocentesView).await?;
    docente_service::get_all_detalle(state).await
}

pub async fn eliminar_docente(state: &AppState, window_label: &str, id_docente: &str) -> Result<EliminarDocenteResultado, AppError> {
    require_permission(state, window_label, AppPermission::DocentesManage).await?;
    docente_service::delete(state, id_docente).await
}

pub async fn reactivar_docente(state: &AppState, window_label: &str, id_docente: &str) -> Result<Docente, AppError> {
    require_permission(state, window_label, AppPermission::DocentesManage).await?;
    docente_service::reactivate(state, id_docente).await
}

pub async fn refrescar_formacion_academica_renacyt_docente(
    state: &AppState,
    window_label: &str,
    id_docente: &str,
) -> Result<RefreshDocenteRenacytFormacionResultado, AppError> {
    require_permission(state, window_label, AppPermission::DocentesManage).await?;
    let mut docente = docente_service::get_by_id(state, id_docente).await?;

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

    docente_service::update_renacyt(state, &docente).await?;

    let docente_detalle = docente_service::get_detalle_by_id(state, id_docente).await?;

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
    proyecto_service::create(state, request).await
}

pub async fn update_proyecto_con_participantes(
    state: &AppState,
    window_label: &str,
    id_proyecto: &str,
    request: UpdateProyectoConParticipantesRequest,
) -> Result<Proyecto, AppError> {
    require_permission(state, window_label, AppPermission::ProyectosManage).await?;
    proyecto_service::update(state, id_proyecto, request).await
}

pub async fn buscar_proyectos_por_docente(state: &AppState, window_label: &str, id_docente: &str) -> Result<Vec<Proyecto>, AppError> {
    require_permission(state, window_label, AppPermission::ProyectosView).await?;
    proyecto_service::find_by_docente(state, id_docente).await
}

pub async fn get_all_proyectos_detalle(state: &AppState, window_label: &str) -> Result<Vec<ProyectoDetalle>, AppError> {
    require_permission(state, window_label, AppPermission::ProyectosView).await?;
    proyecto_service::get_all_detalle(state).await
}

pub async fn eliminar_relacion_proyecto_docente(state: &AppState, window_label: &str, id_proyecto: &str, id_docente: &str) -> Result<(), AppError> {
    require_permission(state, window_label, AppPermission::ProyectosManage).await?;
    proyecto_service::delete_relation(state, id_proyecto, id_docente).await
}

pub async fn eliminar_relaciones_proyecto(state: &AppState, window_label: &str, id_proyecto: &str) -> Result<(), AppError> {
    require_permission(state, window_label, AppPermission::ProyectosManage).await?;
    proyecto_service::delete_relations(state, id_proyecto).await
}

pub async fn eliminar_proyecto(state: &AppState, window_label: &str, id_proyecto: &str) -> Result<EliminarProyectoResultado, AppError> {
    require_permission(state, window_label, AppPermission::ProyectosManage).await?;
    proyecto_service::delete(state, id_proyecto).await
}

pub async fn reactivar_proyecto(state: &AppState, window_label: &str, id_proyecto: &str) -> Result<Proyecto, AppError> {
    require_permission(state, window_label, AppPermission::ProyectosManage).await?;
    proyecto_service::reactivate(state, id_proyecto).await
}

pub async fn get_estadisticas_proyectos_x_docente(state: &AppState, window_label: &str) -> Result<Vec<DocenteProyectosCount>, AppError> {
    require_permission(state, window_label, AppPermission::DashboardView).await?;
    proyecto_service::get_estadisticas_x_docente(state).await
}

pub async fn get_kpis_dashboard(state: &AppState, window_label: &str) -> Result<KpisDashboard, AppError> {
    require_permission(state, window_label, AppPermission::DashboardView).await?;
    proyecto_service::get_kpis(state).await
}

pub async fn get_data_exportacion_plana(state: &AppState, window_label: &str) -> Result<Vec<ExportData>, AppError> {
    require_permission(state, window_label, AppPermission::ReportesExport).await?;
    proyecto_service::get_exportacion_plana(state).await
}

pub async fn get_data_exportacion_agrupada_docente(state: &AppState, window_label: &str) -> Result<Vec<ExportDataConProjectos>, AppError> {
    require_permission(state, window_label, AppPermission::ReportesView).await?;
    proyecto_service::get_exportacion_agrupada(state).await
}

pub async fn crear_usuario(state: &AppState, window_label: &str, request: CreateUsuarioRequest) -> Result<Usuario, AppError> {
    let actor = get_session_actor_user(state, window_label).await?;
    let usuario = usuario_service::create(state, &actor.id_usuario, request).await?;

    crate::audit::write_user_audit(&actor, "usuario.create", &usuario, format!("rol={}", usuario.rol));
    Ok(usuario)
}

pub async fn get_auth_status(state: &AppState) -> Result<AuthStatus, AppError> {
    usuario_service::get_auth_status(state).await
}

pub async fn registrar_primer_usuario(state: &AppState, window_label: &str, request: BootstrapUsuarioRequest) -> Result<Usuario, AppError> {
    let usuario = usuario_service::bootstrap_admin(state, request).await?;

    state.set_current_session(window_label, usuario.id_usuario.clone()).await;
    Ok(usuario)
}

pub async fn login_usuario(state: &AppState, window_label: &str, request: LoginUsuarioRequest) -> Result<Usuario, AppError> {
    let usuario = usuario_service::login(state, request).await?;

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
    usuario_service::get_all(state, &actor.id_usuario).await
}

pub async fn actualizar_usuario(state: &AppState, window_label: &str, id_usuario: &str, request: UpdateUsuarioRequest) -> Result<Usuario, AppError> {
    let actor = get_session_actor_user(state, window_label).await?;
    let previous_user = get_user_by_id(state, id_usuario).await?;
    let usuario = usuario_service::update(state, &actor.id_usuario, id_usuario, request).await?;

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
    let usuario = usuario_service::deactivate(state, &actor.id_usuario, id_usuario).await?;

    crate::audit::write_user_audit(&actor, "usuario.deactivate", &usuario, "activo=0".to_string());
    Ok(usuario)
}

pub async fn reactivar_usuario(state: &AppState, window_label: &str, id_usuario: &str) -> Result<Usuario, AppError> {
    let actor = get_session_actor_user(state, window_label).await?;
    let usuario = usuario_service::reactivate(state, &actor.id_usuario, id_usuario).await?;

    crate::audit::write_user_audit(&actor, "usuario.reactivate", &usuario, "activo=1".to_string());
    Ok(usuario)
}