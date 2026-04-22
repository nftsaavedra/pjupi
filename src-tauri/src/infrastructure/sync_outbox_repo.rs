use chrono::Utc;
use sqlx::{query, query_as, FromRow, SqlitePool};

use crate::error::AppError;

#[derive(Debug, Clone, FromRow)]
pub struct SyncOutboxItem {
    pub id: i64,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub operation_type: String,
    pub payload_json: String,
    pub retry_count: i64,
    pub created_at: i64,
}

pub async fn enqueue_operation(
    pool: &SqlitePool,
    aggregate_type: &str,
    aggregate_id: &str,
    operation_type: &str,
    payload_json: &str,
) -> Result<(), AppError> {
    let now = Utc::now().timestamp_millis();

    query(
        r#"
        INSERT INTO sync_outbox (
            aggregate_type,
            aggregate_id,
            operation_type,
            payload_json,
            status,
            retry_count,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?, 'pending', 0, ?, ?)
        "#,
    )
    .bind(aggregate_type)
    .bind(aggregate_id)
    .bind(operation_type)
    .bind(payload_json)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_pending_operations(pool: &SqlitePool, limit: i64) -> Result<Vec<SyncOutboxItem>, AppError> {
    let capped_limit = limit.clamp(1, 500);

    let operations = query_as::<_, SyncOutboxItem>(
        r#"
        SELECT
            id,
            aggregate_type,
            aggregate_id,
            operation_type,
            payload_json,
            retry_count
        FROM sync_outbox
        WHERE status = 'pending'
        ORDER BY created_at ASC
        LIMIT ?
        "#,
    )
    .bind(capped_limit)
    .fetch_all(pool)
    .await?;

    Ok(operations)
}

pub async fn mark_operation_completed(pool: &SqlitePool, outbox_id: i64) -> Result<(), AppError> {
    let now = Utc::now().timestamp_millis();

    query(
        r#"
        UPDATE sync_outbox
        SET status = 'processed',
            updated_at = ?,
            last_error = NULL
        WHERE id = ?
        "#,
    )
    .bind(now)
    .bind(outbox_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn mark_operation_failed(pool: &SqlitePool, outbox_id: i64, error_message: &str) -> Result<(), AppError> {
    let now = Utc::now().timestamp_millis();

    query(
        r#"
        UPDATE sync_outbox
        SET retry_count = retry_count + 1,
            updated_at = ?,
            last_error = ?,
            status = CASE
                WHEN retry_count + 1 >= 5 THEN 'failed'
                ELSE 'pending'
            END
        WHERE id = ?
        "#,
    )
    .bind(now)
    .bind(error_message)
    .bind(outbox_id)
    .execute(pool)
    .await?;

    Ok(())
}