use std::collections::HashMap;

use chrono::Utc;
use mongodb::Database;
use sqlx::SqlitePool;
use tokio::sync::RwLock;

use crate::config::{DatabaseBackend, PureConfig, RenacytConfig, ReniecConfig};
use crate::error::AppError;

#[derive(Clone)]
pub struct SessionEntry {
    pub user_id: String,
    pub last_activity_at: i64,
}

pub struct SessionStore {
    sessions: RwLock<HashMap<String, SessionEntry>>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
        }
    }

    pub async fn set_user_session(&self, window_label: &str, user_id: String) {
        let now = Utc::now().timestamp();
        let mut sessions = self.sessions.write().await;
        sessions.insert(
            window_label.to_string(),
            SessionEntry {
                user_id,
                last_activity_at: now,
            },
        );
    }

    pub async fn get_user_session(&self, window_label: &str) -> Option<SessionEntry> {
        let sessions = self.sessions.read().await;
        sessions.get(window_label).cloned()
    }

    pub async fn touch_user_session(&self, window_label: &str) {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(window_label) {
            session.last_activity_at = Utc::now().timestamp();
        }
    }

    pub async fn clear_user_session(&self, window_label: &str) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(window_label);
    }
}

pub struct AppState {
    pub primary_backend: DatabaseBackend,
    pub sqlite: Option<SqlitePool>,
    pub mongo: Option<Database>,
    pub reniec: ReniecConfig,
    pub renacyt: RenacytConfig,
    pub pure_config: PureConfig,
    sessions: SessionStore,
}

impl AppState {
    pub fn new(
        primary_backend: DatabaseBackend,
        sqlite: Option<SqlitePool>,
        mongo: Option<Database>,
        reniec: ReniecConfig,
        renacyt: RenacytConfig,
        pure_config: PureConfig,
    ) -> Self {
        Self {
            primary_backend,
            sqlite,
            mongo,
            reniec,
            renacyt,
            pure_config,
            sessions: SessionStore::new(),
        }
    }

    pub fn sqlite_pool(&self) -> Result<&SqlitePool, AppError> {
        self.sqlite.as_ref().ok_or_else(|| {
            AppError::ConfigurationError("SQLite no está inicializado para la configuración actual.".to_string())
        })
    }

    #[allow(dead_code)]
    pub fn mongo_db(&self) -> Result<&Database, AppError> {
        self.mongo.as_ref().ok_or_else(|| {
            AppError::ConfigurationError("MongoDB no está inicializado para la configuración actual.".to_string())
        })
    }

    pub fn reniec_config(&self) -> &ReniecConfig {
        &self.reniec
    }

    pub fn renacyt_config(&self) -> &RenacytConfig {
        &self.renacyt
    }

    pub async fn set_current_session(&self, window_label: &str, user_id: String) {
        self.sessions.set_user_session(window_label, user_id).await;
    }

    pub async fn get_current_session_user_id(&self, window_label: &str) -> Option<String> {
        self.sessions
            .get_user_session(window_label)
            .await
            .map(|session| session.user_id)
    }

    pub async fn touch_current_session(&self, window_label: &str) {
        self.sessions.touch_user_session(window_label).await;
    }

    pub async fn clear_current_session(&self, window_label: &str) {
        self.sessions.clear_user_session(window_label).await;
    }
}