use serde::{Deserialize, Serialize};
use std::fmt;

const MAX_EXTERNAL_DETAIL_LEN: usize = 512;

pub fn sanitize_external_detail(input: &str) -> String {
    let mut sanitized = input.replace("\r", " ").replace("\n", " ");

    for marker in [
        "api-key",
        "apikey",
        "api_key",
        "authorization",
        "bearer ",
        "token",
        "password",
        "secret",
        "PJUPI_PURE_API_KEY",
        "PURE_API_KEY",
        "PJUPI_RENIEC_TOKEN",
    ] {
        sanitized = redact_after_marker(&sanitized, marker);
    }

    if sanitized.len() > MAX_EXTERNAL_DETAIL_LEN {
        sanitized.truncate(MAX_EXTERNAL_DETAIL_LEN);
        sanitized.push_str("...");
    }

    sanitized
}

fn redact_after_marker(input: &str, marker: &str) -> String {
    let lower_input = input.to_lowercase();
    let lower_marker = marker.to_lowercase();
    let mut start = 0usize;
    let mut result = String::with_capacity(input.len());

    while let Some(pos) = lower_input[start..].find(&lower_marker) {
        let marker_start = start + pos;
        let marker_end = marker_start + marker.len();

        result.push_str(&input[start..marker_end]);

        let mut i = marker_end;
        while let Some(ch) = input[i..].chars().next() {
            if ch == ':' || ch == '=' || ch.is_whitespace() {
                result.push(ch);
                i += ch.len_utf8();
            } else {
                break;
            }
        }

        let mut j = i;
        while let Some(ch) = input[j..].chars().next() {
            if ch.is_whitespace() || ch == ',' || ch == ';' || ch == '"' || ch == '\'' {
                break;
            }
            j += ch.len_utf8();
        }

        if j > i {
            result.push_str("[REDACTED]");
            start = j;
        } else {
            start = i;
        }
    }

    result.push_str(&input[start..]);
    result
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AppError {
    DatabaseError(String),
    UniqueConstraintViolation(String),
    NotFound(String),
    InternalError(String),
    ConfigurationError(String),
    ExternalServiceError(String),
}

impl From<mongodb::error::Error> for AppError {
    fn from(err: mongodb::error::Error) -> Self {
        let message = sanitize_external_detail(&err.to_string());
        let lowered = message.to_lowercase();
        if message.contains("E11000") || lowered.contains("duplicate key") {
            let user_message = if lowered.contains("username") {
                "El nombre de usuario ya existe.".to_string()
            } else if lowered.contains("dni") {
                "El DNI ingresado ya existe en el padrón.".to_string()
            } else if lowered.contains("nombre") {
                "Ya existe un registro con ese nombre.".to_string()
            } else {
                "Ya existe un registro con un valor único duplicado.".to_string()
            };
            AppError::UniqueConstraintViolation(user_message)
        } else {
            AppError::DatabaseError(message)
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::ExternalServiceError(sanitize_external_detail(&err.to_string()))
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(message)
            | AppError::UniqueConstraintViolation(message)
            | AppError::NotFound(message)
            | AppError::InternalError(message)
            | AppError::ConfigurationError(message)
            | AppError::ExternalServiceError(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for AppError {}
