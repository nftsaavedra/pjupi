// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::path::Path;

use sqlx::SqlitePool;

mod commands;
mod config;
mod db;
mod domain;
mod error;
mod infrastructure;
mod migration;
mod state;
mod storage;

use commands::docente_cmd::*;
use commands::proyecto_cmd::*;
use commands::reporte_cmd::*;
use commands::grado_cmd::*;
use commands::usuario_cmd::*;
use config::{DatabaseBackend, DatabaseConfig, RenacytConfig, ReniecConfig};
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::async_runtime::block_on(async {
        dotenvy::dotenv().ok();
        let config = DatabaseConfig::from_env();
        let reniec_config = ReniecConfig::from_env();
        let renacyt_config = RenacytConfig::from_env();
        let sqlite_path = config
            .sqlite_url
            .strip_prefix("sqlite:")
            .unwrap_or("database.db");
        let sqlite_exists = Path::new(sqlite_path).exists();

        let sqlite_pool = if config.backend == DatabaseBackend::Sqlite {
            let pool = SqlitePool::connect(&config.sqlite_url)
                .await
                .expect("Failed to connect to SQLite database");
            db::init_db(&pool).await.expect("Failed to initialize SQLite database");
            Some(pool)
        } else {
            None
        };

        let mongo_db = if config.requires_mongodb() {
            let database = db::init_mongo(&config)
                .await
                .expect("Failed to connect to MongoDB");

            if sqlite_exists && !migration::migration_marker_exists(&database)
                .await
                .expect("Failed to inspect SQLite migration marker") {
                let migration_pool = SqlitePool::connect(&config.sqlite_url)
                    .await
                    .expect("Failed to connect to SQLite database for migration");
                db::init_db(&migration_pool)
                    .await
                    .expect("Failed to initialize SQLite database for migration");
                migration::migrate_sqlite_to_mongodb(&migration_pool, &database)
                    .await
                    .expect("Failed to migrate SQLite data to MongoDB");
            }

            Some(database)
        } else {
            None
        };

        let app_state = AppState::new(config.backend, sqlite_pool, mongo_db, reniec_config, renacyt_config);

        tauri::Builder::default()
            .plugin(tauri_plugin_opener::init())
            .manage(app_state)
            .invoke_handler(tauri::generate_handler![
                crear_docente,
                get_all_docentes,
                buscar_docente_por_dni,
                get_all_docentes_con_proyectos,
                eliminar_docente,
                reactivar_docente,
                consultar_dni_reniec,
                consultar_renacyt_docente,
                crear_proyecto_con_participantes,
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
                get_all_grados,
                crear_grado,
                actualizar_grado,
                eliminar_grado,
                reactivar_grado,
                crear_usuario,
                get_auth_status,
                registrar_primer_usuario,
                login_usuario,
                get_all_usuarios,
                actualizar_usuario,
                desactivar_usuario,
                reactivar_usuario
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    });
}
