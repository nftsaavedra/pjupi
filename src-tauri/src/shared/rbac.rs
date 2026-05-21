use crate::usuarios::models::Usuario;
use crate::usuarios::service as usuario_service;
use crate::shared::error::AppError;
use crate::shared::state::AppState;

pub enum AppPermission {
    DashboardView,
    DocentesView,
    DocentesManage,
    ProyectosView,
    ProyectosManage,
    ReportesView,
    ReportesExport,
    GradosRead,
    GradosManage,
    GruposView,
    GruposManage,
    RecursosManage,
    CatalogosManage,
}

pub fn role_has_permission(role: &str, permission: &AppPermission) -> bool {
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
                | AppPermission::GruposView
                | AppPermission::GruposManage
                | AppPermission::RecursosManage
                | AppPermission::CatalogosManage
        ),
        "consulta" => matches!(
            permission,
            AppPermission::DashboardView
                | AppPermission::DocentesView
                | AppPermission::ProyectosView
                | AppPermission::ReportesView
                | AppPermission::GruposView
        ),
        _ => false,
    }
}

pub async fn require_permission(state: &AppState, window_label: &str, permission: AppPermission) -> Result<Usuario, AppError> {
    let actor = get_session_actor_user(state, window_label).await?;

    if !role_has_permission(&actor.rol, &permission) {
        return Err(AppError::InternalError("No tiene permisos para ejecutar esta operación.".to_string()));
    }

    Ok(actor)
}

pub async fn require_docentes_manage_permission(state: &AppState, window_label: &str) -> Result<Usuario, AppError> {
    require_permission(state, window_label, AppPermission::DocentesManage).await
}

pub async fn require_docentes_view_permission(state: &AppState, window_label: &str) -> Result<Usuario, AppError> {
    require_permission(state, window_label, AppPermission::DocentesView).await
}

pub async fn get_session_actor_user(state: &AppState, window_label: &str) -> Result<Usuario, AppError> {
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

pub(crate) async fn get_user_by_id(state: &AppState, user_id: &str) -> Result<Usuario, AppError> {
    usuario_service::get_by_id_public(state, user_id).await
}
