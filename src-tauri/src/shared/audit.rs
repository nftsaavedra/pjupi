use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use serde::Serialize;

use crate::shared::time;
use crate::usuarios::models::Usuario;

#[derive(Serialize)]
struct AuditEntry<'a> {
    timestamp: String,
    actor_user_id: &'a str,
    actor_username: &'a str,
    actor_role: &'a str,
    action: &'a str,
    target_user_id: &'a str,
    target_username: &'a str,
    target_role: &'a str,
    details: String,
}

#[derive(Serialize)]
struct AuditGenericEntry<'a> {
    timestamp: String,
    actor_user_id: &'a str,
    actor_username: &'a str,
    actor_role: &'a str,
    action: &'a str,
    target_type: &'a str,
    target_id: &'a str,
    details: String,
}

fn resolve_audit_log_path() -> PathBuf {
    if let Ok(path) = std::env::var("PJUPI_AUDIT_LOG_PATH") {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }
    std::env::current_dir()
        .unwrap_or_else(|_| std::env::temp_dir())
        .join("pjupi-audit.log")
}

fn append_audit_line(path: &PathBuf, json_line: &str) {
    if let Some(parent) = path.parent() {
        if let Err(error) = create_dir_all(parent) {
            tracing::warn!("No se pudo preparar el directorio de auditoría: {error}");
            return;
        }
    }
    let mut file = match OpenOptions::new().create(true).append(true).open(path) {
        Ok(file) => file,
        Err(error) => {
            tracing::error!("No se pudo abrir el archivo de auditoría {}: {error}", path.display());
            return;
        }
    };
    if let Err(error) = writeln!(file, "{json_line}") {
        tracing::error!("No se pudo escribir en el archivo de auditoría {}: {error}", path.display());
    }
}

pub fn write_user_audit(actor: &Usuario, action: &str, target: &Usuario, details: String) {
    let path = resolve_audit_log_path();
    let entry = AuditEntry {
        timestamp: time::now_rfc3339(),
        actor_user_id: &actor.id_usuario,
        actor_username: &actor.username,
        actor_role: &actor.rol,
        action,
        target_user_id: &target.id_usuario,
        target_username: &target.username,
        target_role: &target.rol,
        details,
    };
    let serialized = match serde_json::to_string(&entry) {
        Ok(value) => value,
        Err(error) => {
            tracing::error!("No se pudo serializar la auditoría: {error}");
            return;
        }
    };
    append_audit_line(&path, &serialized);
}

pub fn write_generic_audit(
    actor: &Usuario,
    action: &str,
    target_type: &str,
    target_id: &str,
    details: String,
) {
    let path = resolve_audit_log_path();
    let entry = AuditGenericEntry {
        timestamp: time::now_rfc3339(),
        actor_user_id: &actor.id_usuario,
        actor_username: &actor.username,
        actor_role: &actor.rol,
        action,
        target_type,
        target_id,
        details,
    };
    let serialized = match serde_json::to_string(&entry) {
        Ok(value) => value,
        Err(error) => {
            tracing::error!("No se pudo serializar la auditoría genérica: {error}");
            return;
        }
    };
    append_audit_line(&path, &serialized);
}
