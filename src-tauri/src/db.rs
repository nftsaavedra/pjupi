use mongodb::{Client, Database};

use crate::config::DatabaseConfig;
use crate::error::AppError;
use crate::infrastructure::mongo_repo;

pub async fn init_mongo(config: &DatabaseConfig) -> Result<Database, AppError> {
    let uri = config.mongodb_uri.as_deref().ok_or_else(|| {
        AppError::ConfigurationError("Falta configurar PJUPI_MONGODB_URI para usar MongoDB.".to_string())
    })?;

    let client = Client::with_uri_str(uri).await?;
    let database = client.database(&config.mongodb_db_name);
    mongo_repo::ensure_indexes(&database).await?;
    Ok(database)
}
