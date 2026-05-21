use tauri::{path::BaseDirectory, Manager};

mod shared;
mod catalogos;
mod docentes;
mod proyectos;
mod grados;
mod usuarios;
mod grupos;
mod recursos;
mod reportes;
mod seguridad;

use docentes::commands as docente_cmds;
use catalogos::commands as catalogo_cmds;
use proyectos::commands as proyecto_cmds;
use reportes::commands as reporte_cmds;
use grados::commands as grado_cmds;
use usuarios::commands as usuario_cmds;
use seguridad::commands as security_cmds;
use grupos::commands as grupo_cmds;
use recursos::commands as recurso_cmds;

// Keep config/config_validator via shared
use shared::config::load_runtime_config;
use shared::config_validator::validate_database_config;
use shared::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();
    shared::logging::init_logging();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let bundled_default_env_path = app.path()
                .resolve("config/default.env", BaseDirectory::Resource)
                .ok();
            let user_config_path = app.path()
                .app_config_dir()
                .unwrap_or_else(|_| app.path().app_data_dir().unwrap_or_else(|_| std::env::temp_dir()))
                .join("pjupi.config.json");

            let runtime_config = load_runtime_config(&user_config_path, bundled_default_env_path.as_deref())?;

            if let Err(error) = validate_database_config(&runtime_config.database) {
                let error_msg = format!(
                    "Error de configuración: {}\n\nArchivo de configuración: {:?}\n\nPara re-configurar la aplicación, elimine el archivo de configuración y reinicie.",
                    error,
                    user_config_path
                );
                tracing::error!("{}", error_msg);
                return Err(std::io::Error::other(error_msg).into());
            }

            let mongo_db = if runtime_config.database.requires_mongodb() {
                let database = tauri::async_runtime::block_on(async {
                    shared::db::init_mongo(&runtime_config.database).await
                }).map_err(|error| {
                    std::io::Error::other(format!(
                        "No se pudo conectar a MongoDB.\n\n\
                        Error: {}\n\n\
                        Verifique:\n\
                        1. La URI de MongoDB es correcta (configurada en {:?})\n\
                        2. El servidor MongoDB está ejecutándose\n\
                        3. Las credenciales son correctas\n\
                        4. La base de datos es accesible desde esta máquina",
                        error,
                        user_config_path
                    ))
                })?;

                Some(database)
            } else {
                None
            };

            if let Some(ref db) = mongo_db {
                tauri::async_runtime::block_on(async {
                    catalogos::repository::seed_catalogos(db).await
                }).map_err(|e| std::io::Error::other(format!("Error sembrando catálogos: {}", e)))?;
            }

            app.manage(AppState::new(
                mongo_db,
                runtime_config.reniec,
                runtime_config.renacyt,
                runtime_config.pure,
            ));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Docentes
            docente_cmds::crear_docente,
            docente_cmds::get_all_docentes,
            docente_cmds::get_all_docentes_paginated,
            docente_cmds::buscar_docente_por_dni,
            docente_cmds::get_all_docentes_con_proyectos,
            docente_cmds::eliminar_docente,
            docente_cmds::reactivar_docente,
            docente_cmds::consultar_dni_reniec,
            docente_cmds::consultar_renacyt_docente,
            docente_cmds::refrescar_formacion_academica_renacyt_docente,
            // Proyectos
            proyecto_cmds::crear_proyecto_con_participantes,
            proyecto_cmds::actualizar_proyecto_con_participantes,
            proyecto_cmds::buscar_proyectos_por_docente,
            proyecto_cmds::get_all_proyectos_detalle,
            proyecto_cmds::eliminar_relacion_proyecto_docente,
            proyecto_cmds::eliminar_relaciones_proyecto,
            proyecto_cmds::eliminar_proyecto,
            proyecto_cmds::reactivar_proyecto,
            // Reportes
            reporte_cmds::get_estadisticas_proyectos_x_docente,
            reporte_cmds::get_kpis_dashboard,
            reporte_cmds::get_data_exportacion_plana,
            reporte_cmds::get_data_exportacion_agrupada_docente,
            reporte_cmds::write_export_file,
            reporte_cmds::get_reporte_proyecto_integral,
            reporte_cmds::get_reporte_docente_integral,
            reporte_cmds::get_reportes_docentes_integral,
            reporte_cmds::get_data_exportacion_grupos,
            reporte_cmds::get_data_exportacion_recursos,
            reporte_cmds::get_data_exportacion_docentes_perfil,
            reporte_cmds::get_data_exportacion_proyectos_area,
            reporte_cmds::get_proyectos_trend,
            reporte_cmds::get_renacyt_distribucion,
            // Grados
            grado_cmds::get_all_grados,
            grado_cmds::crear_grado,
            grado_cmds::actualizar_grado,
            grado_cmds::eliminar_grado,
            grado_cmds::reactivar_grado,
            // Catálogos
            catalogo_cmds::get_catalogos,
            catalogo_cmds::get_all_catalogos_admin,
            catalogo_cmds::crear_catalogo,
            catalogo_cmds::actualizar_catalogo,
            catalogo_cmds::eliminar_catalogo,
            catalogo_cmds::reactivar_catalogo,
            // Usuarios
            usuario_cmds::crear_usuario,
            usuario_cmds::get_auth_status,
            usuario_cmds::registrar_primer_usuario,
            usuario_cmds::login_usuario,
            usuario_cmds::get_current_session,
            usuario_cmds::logout_usuario,
            usuario_cmds::get_all_usuarios,
            usuario_cmds::actualizar_usuario,
            usuario_cmds::desactivar_usuario,
            usuario_cmds::reactivar_usuario,
            // Seguridad
            security_cmds::get_security_status,
            security_cmds::get_setup_guide,
            security_cmds::get_security_recommendations,
            // Pure (moved to shared/external)
            crate::shared::external::pure_cmd::sincronizar_publicaciones_pure,
            crate::shared::external::pure_cmd::get_publicaciones_docente,
            // Grupos
            grupo_cmds::get_all_grupos,
            grupo_cmds::create_grupo,
            grupo_cmds::get_grupo,
            grupo_cmds::update_grupo,
            grupo_cmds::delete_grupo,
            // Recursos
            recurso_cmds::crear_patente,
            recurso_cmds::get_patentes_proyecto,
            recurso_cmds::actualizar_patente,
            recurso_cmds::eliminar_patente,
            recurso_cmds::crear_producto,
            recurso_cmds::get_productos_proyecto,
            recurso_cmds::actualizar_producto,
            recurso_cmds::eliminar_producto,
            recurso_cmds::crear_equipamiento,
            recurso_cmds::get_equipamientos_proyecto,
            recurso_cmds::actualizar_equipamiento,
            recurso_cmds::eliminar_equipamiento,
            recurso_cmds::crear_financiamiento,
            recurso_cmds::get_financiamientos_proyecto,
            recurso_cmds::actualizar_financiamiento,
            recurso_cmds::eliminar_financiamiento,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
