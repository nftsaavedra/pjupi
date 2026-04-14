use sqlx::SqlitePool;
use tauri::{path::BaseDirectory, Manager};

mod commands;
mod config;
mod db;
mod domain;
mod error;
mod infrastructure;
mod audit;
mod state;
mod storage;

use commands::docente_cmd::*;
use commands::proyecto_cmd::*;
use commands::reporte_cmd::*;
use commands::grado_cmd::*;
use commands::usuario_cmd::*;
use config::{load_runtime_config, DatabaseBackend};
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

            let sqlite_pool = if runtime_config.database.backend == DatabaseBackend::Sqlite {
                let pool = tauri::async_runtime::block_on(async {
                    let pool = SqlitePool::connect(&runtime_config.database.sqlite_url).await?;
                    db::init_db(&pool).await?;
                    Ok::<SqlitePool, sqlx::Error>(pool)
                }).map_err(|error| {
                    std::io::Error::other(format!("No se pudo inicializar SQLite: {}", error))
                })?;
                Some(pool)
            } else {
                None
            };

            let mongo_db = if runtime_config.database.requires_mongodb() {
                let database = tauri::async_runtime::block_on(async {
                    db::init_mongo(&runtime_config.database).await
                }).map_err(|error| {
                    std::io::Error::other(format!("No se pudo conectar a MongoDB. Revise la configuración en {:?}: {}", runtime_config.user_env_path, error))
                })?;

                Some(database)
            } else {
                None
            };

            app.manage(AppState::new(
                runtime_config.database.backend,
                sqlite_pool,
                mongo_db,
                runtime_config.reniec,
                runtime_config.renacyt,
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
            reactivar_usuario
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
