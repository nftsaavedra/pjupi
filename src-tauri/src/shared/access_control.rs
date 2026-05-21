use crate::docentes::models::{CreateDocenteRequest, Docente, DocenteDetalle, EliminarDocenteResultado, RefreshDocenteRenacytFormacionResultado};
use crate::proyectos::models::{DocenteProyectosCount, KpisDashboard, ProyectosTrendItem, RenacytDistribucionItem};
use crate::reportes::models::{ExportData, ExportDataConProjectos, ExportDataDocentePerfil, ExportDataGrupo, ExportDataProyectoArea, ExportDataRecurso};
use crate::recursos::models::{CreateEquipamientoRequest, Equipamiento, UpdateEquipamientoRequest};
use crate::recursos::models::{CreateFinanciamientoRequest, Financiamiento, UpdateFinanciamientoRequest};
use crate::grados::models::{CreateGradoRequest, EliminarGradoResultado, GradoAcademico};
use crate::grupos::models::{GrupoInvestigacion, CreateGrupoInvestigacionRequest, UpdateGrupoInvestigacionRequest};
use crate::recursos::models::{CreatePatenteRequest, Patente, UpdatePatenteRequest};
use crate::recursos::models::{CreateProductoRequest, Producto, UpdateProductoRequest};
use crate::proyectos::models::{
    CreateProyectoConParticipantesRequest,
    EliminarProyectoResultado,
    Proyecto,
    ProyectoDetalle,
    UpdateProyectoConParticipantesRequest,
};
use crate::usuarios::models::{AuthStatus, BootstrapUsuarioRequest, CreateUsuarioRequest, LoginUsuarioRequest, UpdateUsuarioRequest, Usuario};
use crate::catalogos::models::{CatalogoItem, CreateCatalogoRequest, EliminarCatalogoResultado};
use crate::reportes::entity_reports::{ReporteProyectoIntegral, ReporteDocenteIntegral};
use crate::shared::error::AppError;
use crate::docentes::service as docente_service;
use crate::grados::service as grado_service;
use crate::proyectos::service as proyecto_service;
use crate::usuarios::service as usuario_service;
use crate::recursos::service as recurso_service;
use crate::catalogos::service as catalogo_service;
use crate::shared::state::AppState;
use crate::shared::pagination::PaginatedResult;
use crate::shared::rbac;

pub use crate::shared::rbac::{require_docentes_manage_permission, require_docentes_view_permission};

pub async fn get_all_grados(state: &AppState, window_label: &str) -> Result<Vec<GradoAcademico>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::GradosRead).await?;
    grado_service::get_all(state).await
}

pub async fn crear_grado(state: &AppState, window_label: &str, request: CreateGradoRequest) -> Result<GradoAcademico, AppError> {
    let actor = rbac::require_permission(state, window_label, rbac::AppPermission::GradosManage).await?;
    let grado = grado_service::create(state, request).await?;
    crate::shared::audit::write_generic_audit(&actor, "grado.create", "grado", &grado.id_grado, format!("nombre: {}", grado.nombre));
    Ok(grado)
}

pub async fn actualizar_grado(state: &AppState, window_label: &str, id_grado: &str, request: CreateGradoRequest) -> Result<GradoAcademico, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::GradosManage).await?;
    grado_service::update(state, id_grado, request).await
}

pub async fn eliminar_grado(state: &AppState, window_label: &str, id_grado: &str) -> Result<EliminarGradoResultado, AppError> {
    let actor = rbac::require_permission(state, window_label, rbac::AppPermission::GradosManage).await?;
    let result = grado_service::delete(state, id_grado).await?;
    crate::shared::audit::write_generic_audit(&actor, "grado.delete", "grado", id_grado, result.accion.clone());
    Ok(result)
}

pub async fn reactivar_grado(state: &AppState, window_label: &str, id_grado: &str) -> Result<GradoAcademico, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::GradosManage).await?;
    grado_service::reactivate(state, id_grado).await
}

pub async fn crear_docente(state: &AppState, window_label: &str, request: CreateDocenteRequest) -> Result<Docente, AppError> {
    let actor = rbac::require_permission(state, window_label, rbac::AppPermission::DocentesManage).await?;
    let docente = docente_service::create(state, request).await?;
    crate::shared::audit::write_generic_audit(&actor, "docente.create", "docente", &docente.dni, docente.nombres_apellidos.clone());
    Ok(docente)
}

pub async fn get_all_docentes(state: &AppState, window_label: &str) -> Result<Vec<Docente>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::DocentesView).await?;
    docente_service::get_all(state).await
}

pub async fn get_all_docentes_paginated(state: &AppState, window_label: &str, page: u32, limit: u32) -> Result<PaginatedResult<Docente>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::DocentesView).await?;
    docente_service::get_all_paginated(state, page, limit).await
}

pub async fn buscar_docente_por_dni(state: &AppState, window_label: &str, dni: &str) -> Result<Option<Docente>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::DocentesManage).await?;
    docente_service::find_by_dni(state, dni).await
}

pub async fn get_all_docentes_con_proyectos(state: &AppState, window_label: &str) -> Result<Vec<DocenteDetalle>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::DocentesView).await?;
    docente_service::get_all_detalle(state).await
}

pub async fn eliminar_docente(state: &AppState, window_label: &str, id_docente: &str) -> Result<EliminarDocenteResultado, AppError> {
    let actor = rbac::require_permission(state, window_label, rbac::AppPermission::DocentesManage).await?;
    let result = docente_service::delete(state, id_docente).await?;
    crate::shared::audit::write_generic_audit(&actor, "docente.delete", "docente", id_docente, result.accion.clone());
    Ok(result)
}

pub async fn reactivar_docente(state: &AppState, window_label: &str, id_docente: &str) -> Result<Docente, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::DocentesManage).await?;
    docente_service::reactivate(state, id_docente).await
}

pub async fn refrescar_formacion_academica_renacyt_docente(
    state: &AppState,
    window_label: &str,
    id_docente: &str,
) -> Result<RefreshDocenteRenacytFormacionResultado, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::DocentesManage).await?;
    docente_service::refresh_renacyt_formacion(state, id_docente).await
}

pub async fn crear_proyecto_con_participantes(state: &AppState, window_label: &str, request: CreateProyectoConParticipantesRequest) -> Result<Proyecto, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ProyectosManage).await?;
    proyecto_service::create(state, request).await
}

pub async fn update_proyecto_con_participantes(
    state: &AppState,
    window_label: &str,
    id_proyecto: &str,
    request: UpdateProyectoConParticipantesRequest,
) -> Result<Proyecto, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ProyectosManage).await?;
    proyecto_service::update(state, id_proyecto, request).await
}

pub async fn buscar_proyectos_por_docente(state: &AppState, window_label: &str, id_docente: &str) -> Result<Vec<Proyecto>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ProyectosView).await?;
    proyecto_service::find_by_docente(state, id_docente).await
}

pub async fn get_all_proyectos_detalle(state: &AppState, window_label: &str) -> Result<Vec<ProyectoDetalle>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ProyectosView).await?;
    proyecto_service::get_all_detalle(state).await
}

pub async fn eliminar_relacion_proyecto_docente(state: &AppState, window_label: &str, id_proyecto: &str, id_docente: &str) -> Result<(), AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ProyectosManage).await?;
    proyecto_service::delete_relation(state, id_proyecto, id_docente).await
}

pub async fn eliminar_relaciones_proyecto(state: &AppState, window_label: &str, id_proyecto: &str) -> Result<(), AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ProyectosManage).await?;
    proyecto_service::delete_relations(state, id_proyecto).await
}

pub async fn eliminar_proyecto(state: &AppState, window_label: &str, id_proyecto: &str) -> Result<EliminarProyectoResultado, AppError> {
    let actor = rbac::require_permission(state, window_label, rbac::AppPermission::ProyectosManage).await?;
    let result = proyecto_service::delete(state, id_proyecto).await?;
    crate::shared::audit::write_generic_audit(&actor, "proyecto.delete", "proyecto", id_proyecto, result.mensaje.clone());
    Ok(result)
}

pub async fn reactivar_proyecto(state: &AppState, window_label: &str, id_proyecto: &str) -> Result<Proyecto, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ProyectosManage).await?;
    proyecto_service::reactivate(state, id_proyecto).await
}

pub async fn get_estadisticas_proyectos_x_docente(state: &AppState, window_label: &str) -> Result<Vec<DocenteProyectosCount>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::DashboardView).await?;
    proyecto_service::get_estadisticas_x_docente(state).await
}

pub async fn get_kpis_dashboard(state: &AppState, window_label: &str) -> Result<KpisDashboard, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::DashboardView).await?;
    proyecto_service::get_kpis(state).await
}

pub async fn get_data_exportacion_plana(state: &AppState, window_label: &str) -> Result<Vec<ExportData>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ReportesExport).await?;
    proyecto_service::get_exportacion_plana(state).await
}

pub async fn get_data_exportacion_agrupada_docente(state: &AppState, window_label: &str) -> Result<Vec<ExportDataConProjectos>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ReportesView).await?;
    proyecto_service::get_exportacion_agrupada(state).await
}

pub async fn crear_usuario(state: &AppState, window_label: &str, request: CreateUsuarioRequest) -> Result<Usuario, AppError> {
    let actor = rbac::get_session_actor_user(state, window_label).await?;
    let usuario = usuario_service::create(state, &actor.id_usuario, request).await?;

    crate::shared::audit::write_user_audit(&actor, "usuario.create", &usuario, format!("rol={}", usuario.rol));
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

    let actor = match rbac::get_user_by_id(state, &actor_user_id).await {
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


// ── Patentes ──────────────────────────────────────────────────────────────────

pub async fn crear_patente(state: &AppState, window_label: &str, request: CreatePatenteRequest) -> Result<Patente, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::RecursosManage).await?;
    recurso_service::create_patente(state, request).await
}

pub async fn get_patentes_proyecto(state: &AppState, window_label: &str, proyecto_id: &str) -> Result<Vec<Patente>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ProyectosView).await?;
    recurso_service::get_patentes_by_proyecto(state, proyecto_id).await
}

pub async fn actualizar_patente(state: &AppState, window_label: &str, id_patente: &str, request: UpdatePatenteRequest) -> Result<Patente, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::RecursosManage).await?;
    recurso_service::update_patente(state, id_patente, request).await
}

pub async fn eliminar_patente(state: &AppState, window_label: &str, id_patente: &str) -> Result<(), AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::RecursosManage).await?;
    recurso_service::delete_patente(state, id_patente).await
}

// ── Productos ─────────────────────────────────────────────────────────────────

pub async fn crear_producto(state: &AppState, window_label: &str, request: CreateProductoRequest) -> Result<Producto, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::RecursosManage).await?;
    recurso_service::create_producto(state, request).await
}

pub async fn get_productos_proyecto(state: &AppState, window_label: &str, proyecto_id: &str) -> Result<Vec<Producto>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ProyectosView).await?;
    recurso_service::get_productos_by_proyecto(state, proyecto_id).await
}

pub async fn actualizar_producto(state: &AppState, window_label: &str, id_producto: &str, request: UpdateProductoRequest) -> Result<Producto, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::RecursosManage).await?;
    recurso_service::update_producto(state, id_producto, request).await
}

pub async fn eliminar_producto(state: &AppState, window_label: &str, id_producto: &str) -> Result<(), AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::RecursosManage).await?;
    recurso_service::delete_producto(state, id_producto).await
}

// ── Equipamientos ─────────────────────────────────────────────────────────────

pub async fn crear_equipamiento(state: &AppState, window_label: &str, request: CreateEquipamientoRequest) -> Result<Equipamiento, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::RecursosManage).await?;
    recurso_service::create_equipamiento(state, request).await
}

pub async fn get_equipamientos_proyecto(state: &AppState, window_label: &str, proyecto_id: &str) -> Result<Vec<Equipamiento>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ProyectosView).await?;
    recurso_service::get_equipamientos_by_proyecto(state, proyecto_id).await
}

pub async fn actualizar_equipamiento(state: &AppState, window_label: &str, id_equipamiento: &str, request: UpdateEquipamientoRequest) -> Result<Equipamiento, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::RecursosManage).await?;
    recurso_service::update_equipamiento(state, id_equipamiento, request).await
}

pub async fn eliminar_equipamiento(state: &AppState, window_label: &str, id_equipamiento: &str) -> Result<(), AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::RecursosManage).await?;
    recurso_service::delete_equipamiento(state, id_equipamiento).await
}

// ── Financiamientos ───────────────────────────────────────────────────────────

pub async fn crear_financiamiento(state: &AppState, window_label: &str, request: CreateFinanciamientoRequest) -> Result<Financiamiento, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::RecursosManage).await?;
    recurso_service::create_financiamiento(state, request).await
}

pub async fn get_financiamientos_proyecto(state: &AppState, window_label: &str, proyecto_id: &str) -> Result<Vec<Financiamiento>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ProyectosView).await?;
    recurso_service::get_financiamientos_by_proyecto(state, proyecto_id).await
}

pub async fn actualizar_financiamiento(state: &AppState, window_label: &str, id_financiamiento: &str, request: UpdateFinanciamientoRequest) -> Result<Financiamiento, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::RecursosManage).await?;
    recurso_service::update_financiamiento(state, id_financiamiento, request).await
}

pub async fn eliminar_financiamiento(state: &AppState, window_label: &str, id_financiamiento: &str) -> Result<(), AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::RecursosManage).await?;
    recurso_service::delete_financiamiento(state, id_financiamiento).await
}

pub async fn get_all_usuarios(state: &AppState, window_label: &str) -> Result<Vec<Usuario>, AppError> {
    let actor = rbac::get_session_actor_user(state, window_label).await?;
    usuario_service::get_all(state, &actor.id_usuario).await
}

pub async fn actualizar_usuario(state: &AppState, window_label: &str, id_usuario: &str, request: UpdateUsuarioRequest) -> Result<Usuario, AppError> {
    let actor = rbac::get_session_actor_user(state, window_label).await?;
    let previous_user = rbac::get_user_by_id(state, id_usuario).await?;
    let usuario = usuario_service::update(state, &actor.id_usuario, id_usuario, request).await?;

    crate::shared::audit::write_user_audit(
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
    let actor = rbac::get_session_actor_user(state, window_label).await?;
    let usuario = usuario_service::deactivate(state, &actor.id_usuario, id_usuario).await?;

    crate::shared::audit::write_user_audit(&actor, "usuario.deactivate", &usuario, "activo=0".to_string());
    Ok(usuario)
}

pub async fn reactivar_usuario(state: &AppState, window_label: &str, id_usuario: &str) -> Result<Usuario, AppError> {
    let actor = rbac::get_session_actor_user(state, window_label).await?;
    let usuario = usuario_service::reactivate(state, &actor.id_usuario, id_usuario).await?;

    crate::shared::audit::write_user_audit(&actor, "usuario.reactivate", &usuario, "activo=1".to_string());
    Ok(usuario)
}

pub async fn get_all_grupos(state: &AppState, window_label: &str) -> Result<Vec<GrupoInvestigacion>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::GruposView).await?;
    crate::grupos::service::get_all(state).await
}

pub async fn create_grupo(state: &AppState, window_label: &str, request: CreateGrupoInvestigacionRequest) -> Result<GrupoInvestigacion, AppError> {
    let actor = rbac::require_permission(state, window_label, rbac::AppPermission::GruposManage).await?;
    let grupo = crate::grupos::service::create(state, request).await?;
    crate::shared::audit::write_generic_audit(&actor, "grupo.create", "grupo", &grupo.id_grupo, grupo.nombre.clone());
    Ok(grupo)
}

pub async fn get_grupo(state: &AppState, window_label: &str, id_grupo: &str) -> Result<GrupoInvestigacion, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::GruposView).await?;
    crate::grupos::service::get_by_id(state, id_grupo).await
}

pub async fn update_grupo(state: &AppState, window_label: &str, id_grupo: &str, request: UpdateGrupoInvestigacionRequest) -> Result<GrupoInvestigacion, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::GruposManage).await?;
    crate::grupos::service::update(state, id_grupo, request).await
}

pub async fn delete_grupo(state: &AppState, window_label: &str, id_grupo: &str) -> Result<(), AppError> {
    let actor = rbac::require_permission(state, window_label, rbac::AppPermission::GruposManage).await?;
    crate::grupos::service::delete(state, id_grupo).await?;
    crate::shared::audit::write_generic_audit(&actor, "grupo.delete", "grupo", id_grupo, String::new());
    Ok(())
}

// ── Catálogos ─────────────────────────────────────────────────────────────────

pub async fn get_catalogos(state: &AppState, window_label: &str, tipo: &str) -> Result<Vec<CatalogoItem>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::GradosRead).await?;
    catalogo_service::get_by_tipo(state, tipo).await
}

pub async fn get_all_catalogos_admin(state: &AppState, window_label: &str, tipo: &str) -> Result<Vec<CatalogoItem>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::GradosManage).await?;
    catalogo_service::get_all_by_tipo(state, tipo).await
}

pub async fn crear_catalogo(state: &AppState, window_label: &str, request: CreateCatalogoRequest) -> Result<CatalogoItem, AppError> {
    let actor = rbac::require_permission(state, window_label, rbac::AppPermission::CatalogosManage).await?;
    let item = catalogo_service::create(state, request).await?;
    crate::shared::audit::write_generic_audit(&actor, "catalogo.create", "catalogo", &item.id_catalogo, format!("tipo: {}, codigo: {}", item.tipo, item.codigo));
    Ok(item)
}

pub async fn actualizar_catalogo(state: &AppState, window_label: &str, id: &str, request: CreateCatalogoRequest) -> Result<CatalogoItem, AppError> {
    let actor = rbac::require_permission(state, window_label, rbac::AppPermission::CatalogosManage).await?;
    let item = catalogo_service::update(state, id, request).await?;
    crate::shared::audit::write_generic_audit(&actor, "catalogo.update", "catalogo", id, format!("codigo: {}", item.codigo));
    Ok(item)
}

pub async fn eliminar_catalogo(state: &AppState, window_label: &str, id: &str) -> Result<EliminarCatalogoResultado, AppError> {
    let actor = rbac::require_permission(state, window_label, rbac::AppPermission::CatalogosManage).await?;
    let result = catalogo_service::delete(state, id).await?;
    crate::shared::audit::write_generic_audit(&actor, "catalogo.delete", "catalogo", id, result.accion.clone());
    Ok(result)
}

pub async fn reactivar_catalogo(state: &AppState, window_label: &str, id: &str) -> Result<CatalogoItem, AppError> {
    let actor = rbac::require_permission(state, window_label, rbac::AppPermission::CatalogosManage).await?;
    let item = catalogo_service::reactivate(state, id).await?;
    crate::shared::audit::write_generic_audit(&actor, "catalogo.reactivate", "catalogo", id, item.codigo.clone());
    Ok(item)
}

// ── Exportación de archivos ───────────────────────────────────────────────────

pub async fn write_export_file(state: &AppState, window_label: &str, file_path: &str, bytes: Vec<u8>) -> Result<(), AppError> {
    let actor = rbac::require_permission(state, window_label, rbac::AppPermission::ReportesExport).await?;
    let trimmed_path = file_path.trim();
    if trimmed_path.is_empty() {
        return Err(AppError::ConfigurationError("La ruta de exportacion es invalida.".to_string()));
    }
    if let Some(parent) = std::path::Path::new(trimmed_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|error| {
                AppError::InternalError(format!("No se pudo preparar la carpeta de exportacion: {error}"))
            })?;
        }
    }
    std::fs::write(trimmed_path, &bytes).map_err(|error| {
        AppError::InternalError(format!("No se pudo guardar el archivo exportado: {error}"))
    })?;
    crate::shared::audit::write_generic_audit(&actor, "reportes.export", "archivo", file_path, format!("{} bytes", bytes.len()));
    Ok(())
}

// ── Reportes Integrales de Entidad ────────────────────────────────────────────

pub async fn get_reporte_proyecto_integral(state: &AppState, window_label: &str, id_proyecto: &str) -> Result<ReporteProyectoIntegral, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ReportesExport).await?;
    crate::reportes::entity_service::get_reporte_proyecto(state, id_proyecto).await
}

pub async fn get_reporte_docente_integral(state: &AppState, window_label: &str, id_docente: &str) -> Result<ReporteDocenteIntegral, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ReportesExport).await?;
    crate::reportes::entity_service::get_reporte_docente(state, id_docente).await
}

pub async fn get_reportes_docentes_integral(state: &AppState, window_label: &str) -> Result<Vec<ReporteDocenteIntegral>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ReportesExport).await?;
    crate::reportes::entity_service::get_reportes_docentes(state).await
}

pub async fn get_data_exportacion_grupos(state: &AppState, window_label: &str) -> Result<Vec<ExportDataGrupo>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ReportesView).await?;
    proyecto_service::get_exportacion_grupos(state).await
}

pub async fn get_data_exportacion_recursos(state: &AppState, window_label: &str) -> Result<Vec<ExportDataRecurso>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ReportesView).await?;
    proyecto_service::get_exportacion_recursos(state).await
}

pub async fn get_data_exportacion_docentes_perfil(state: &AppState, window_label: &str) -> Result<Vec<ExportDataDocentePerfil>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ReportesView).await?;
    proyecto_service::get_exportacion_docentes_perfil(state).await
}

pub async fn get_data_exportacion_proyectos_area(state: &AppState, window_label: &str) -> Result<Vec<ExportDataProyectoArea>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::ReportesView).await?;
    proyecto_service::get_exportacion_proyectos_area(state).await
}

pub async fn get_proyectos_trend(state: &AppState, window_label: &str) -> Result<Vec<ProyectosTrendItem>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::DashboardView).await?;
    proyecto_service::get_proyectos_trend(state).await
}

pub async fn get_renacyt_distribucion(state: &AppState, window_label: &str) -> Result<Vec<RenacytDistribucionItem>, AppError> {
    rbac::require_permission(state, window_label, rbac::AppPermission::DashboardView).await?;
    proyecto_service::get_renacyt_distribucion(state).await
}
