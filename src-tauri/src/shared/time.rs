use std::time::{SystemTime, UNIX_EPOCH};

/// Timestamp actual en milisegundos desde UNIX epoch.
#[inline]
pub fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// Timestamp actual como String RFC 3339 (ISO 8601).
/// Centraliza el uso de chrono::Utc para evitar esparcirlo en todo el codebase.
pub fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339()
}
