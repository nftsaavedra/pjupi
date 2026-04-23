use sqlx::SqlitePool;
use tauri::{path::BaseDirectory, Manager};

mod commands;
mod config;
mod config_validator;
mod setup_wizard;
mod db;
mod domain;
mod error;
mod infrastructure;
mod audit;
mod services;
mod state;
mod storage;

use commands::docente_cmd::*;
use commands::proyecto_cmd::*;
use commands::reporte_cmd::*;
use commands::grado_cmd::*;
use commands::usuario_cmd::*;
use commands::security_cmd::*;
use commands::pure_cmd::*;
use commands::grupo_cmd::*;
use config::load_runtime_config;
use config_validator::validate_database_config;
use services::sync_service;
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let bundled_default_env_path = app.path()
                .resolve("config/default.env", BaseDirectory::Resource)
                .ok();
            let user_env_path = app.path()
                .app_config_dir()
                .unwrap_or_else(|_| app.path().app_data_dir().unwrap_or_else(|_| std::env::temp_dir()))
                .join("pjupi.env");

            let runtime_config = load_runtime_config(&user_env_path, bundled_default_env_path.as_deref())?;

            // Validate configuration before attempting to connect
            if let Err(error) = validate_database_config(&runtime_config.database) {
                let error_msg = format!(
                    "Error de configuración: {}\n\nArchivo de configuración: {:?}\n\nPara re-configurar la aplicación, elimine el archivo de configuración y reinicie.",
                    error,
                    user_env_path
                );
                eprintln!("{}", error_msg);
                return Err(std::io::Error::other(error_msg).into());
            }

            let sqlite_pool = if runtime_config.database.should_init_local_sqlite() {
                let pool = tauri::async_runtime::block_on(async {
                    let pool = SqlitePool::connect(&runtime_config.database.sqlite_url).await?;
                    db::init_db(&pool).await?;
                    Ok::<SqlitePool, sqlx::Error>(pool)
                }).map_err(|error| {
                    std::io::Error::other(format!(
                        "No se pudo inicializar SQLite en {}. Error: {}\n\nVerifique permisos de directorio y que la ruta sea válida.",
                        runtime_config.database.sqlite_url,
                        error
                    ))
                })?;
                Some(pool)
            } else {
                None
            };

            let mongo_db = if runtime_config.database.requires_mongodb() {
                let database = tauri::async_runtime::block_on(async {
                    db::init_mongo(&runtime_config.database).await
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
                        user_env_path
                    ))
                })?;

                Some(database)
            } else {
                None
            };

            if let (Some(sqlite), Some(mongo)) = (sqlite_pool.as_ref(), mongo_db.as_ref()) {
                if let Err(error) = tauri::async_runtime::block_on(sync_service::sync_pending_once(sqlite, mongo)) {
                    eprintln!("No se pudo sincronizar outbox local al arrancar: {}", error);
                }
            }

            app.manage(AppState::new(
                runtime_config.database.primary_backend,
                sqlite_pool,
                mongo_db,
                runtime_config.reniec,
                runtime_config.renacyt,
                runtime_config.pure,
            ));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crear_docente,
            get_all_docentes,
            buscar_docente_por_dni,
            get_all_docentes_con_proyectos,
            eliminar_docente,
            reactivar_docente,
            consultar_dni_reniec,
            consultar_renacyt_docente,
            refrescar_formacion_academica_renacyt_docente,
            crear_proyecto_con_participantes,
            actualizar_proyecto_con_participantes,
            buscar_proyectos_por_docente,
            get_all_proyectos_detalle,
            eliminar_relacion_proyecto_docente,
            eliminar_relaciones_proyecto,
            eliminar_proyecto,
            reactivar_proyecto,
            get_estadisticas_proyectos_x_docente,
            get_kpis_dashboard,
            get_data_exportacion_plana,
            get_data_exportacion_agrupada_docente,
            write_export_file,
            get_all_grados,
            crear_grado,
            actualizar_grado,
            eliminar_grado,
            reactivar_grado,
            crear_usuario,
            get_auth_status,
            registrar_primer_usuario,
            login_usuario,
            get_current_session,
            logout_usuario,
            get_all_usuarios,
            actualizar_usuario,
            desactivar_usuario,
            reactivar_usuario,
            get_security_status,
            get_setup_guide,
            get_security_recommendations,
            sincronizar_publicaciones_pure,
            get_publicaciones_docente,
            get_all_grupos,
            create_grupo,
            get_grupo,
            update_grupo,
            delete_grupo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
