use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::common::error::AppError;

use super::model::NoticeRecord;

pub struct NoticeRepository;

impl NoticeRepository {
    pub async fn get_notice_by_id(
        db: &PgPool,
        notice_id: Uuid,
    ) -> Result<Option<NoticeRecord>, AppError> {
        let n = sqlx::query_as::<_, NoticeRecord>(
            r#"
            SELECT 
                id, notice_title, notice_type, notice_content, notice_status, 
                is_top, priority, publish_time, view_count, publisher_id, 
                publisher_name, created_by, updated_by, created_at, 
                updated_at, deleted_at
            FROM sys_notice 
            WHERE id = $1 AND (deleted_at IS NULL OR deleted_at > NOW())
            "#,
        )
        .bind(notice_id)
        .fetch_optional(db)
        .await?;
        Ok(n)
    }

    pub async fn list_notices(
        db: &PgPool,
        notice_type: Option<&str>,
        notice_status: Option<&str>,
    ) -> Result<Vec<NoticeRecord>, AppError> {
        let mut query = String::from(
            r#"
            SELECT 
                id, notice_title, notice_type, notice_content, notice_status, 
                is_top, priority, publish_time, view_count, publisher_id, 
                publisher_name, created_by, updated_by, created_at, 
                updated_at, deleted_at
            FROM sys_notice 
            WHERE deleted_at IS NULL OR deleted_at > NOW()
            "#,
        );
        
        if let Some(t) = notice_type {
            query.push_str(&format!(" AND notice_type = '{}'", t));
        }
        if let Some(s) = notice_status {
            query.push_str(&format!(" AND notice_status = '{}'", s));
        }
        query.push_str(" ORDER BY created_at DESC");
        
        let notices = sqlx::query_as::<_, NoticeRecord>(&query)
            .fetch_all(db)
            .await?;
        Ok(notices)
    }

    pub async fn create_notice(
        db: &PgPool,
        notice_title: String,
        notice_type: String,
        notice_content: Option<String>,
        notice_status: Option<String>,
        is_top: Option<String>,
        priority: Option<i32>,
        publish_time: Option<String>,
    ) -> Result<NoticeRecord, AppError> {
        let publish_time_db = publish_time
            .and_then(|t| chrono::DateTime::parse_from_rfc3339(&t).ok())
            .map(|t| t.with_timezone(&chrono::Utc));
        
        let n = sqlx::query_as::<_, NoticeRecord>(
            r#"
            INSERT INTO sys_notice (
                notice_title, notice_type, notice_content, notice_status, 
                is_top, priority, publish_time
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING 
                id, notice_title, notice_type, notice_content, notice_status, 
                is_top, priority, publish_time, view_count, publisher_id, 
                publisher_name, created_by, updated_by, created_at, 
                updated_at, deleted_at
            "#,
        )
        .bind(notice_title)
        .bind(notice_type)
        .bind(notice_content)
        .bind(notice_status)
        .bind(is_top)
        .bind(priority)
        .bind(publish_time_db)
        .fetch_one(db)
        .await?;
        Ok(n)
    }

    pub async fn update_notice(
        db: &PgPool,
        notice_id: Uuid,
        notice_title: Option<String>,
        notice_type: Option<String>,
        notice_content: Option<String>,
        notice_status: Option<String>,
        is_top: Option<String>,
        priority: Option<i32>,
        publish_time: Option<String>,
    ) -> Result<NoticeRecord, AppError> {
        let publish_time_db = publish_time
            .and_then(|t| chrono::DateTime::parse_from_rfc3339(&t).ok())
            .map(|t| t.with_timezone(&chrono::Utc));
        
        let n = sqlx::query_as::<_, NoticeRecord>(
            r#"
            UPDATE sys_notice 
            SET 
                notice_title = COALESCE($2, notice_title),
                notice_type = COALESCE($3, notice_type),
                notice_content = COALESCE($4, notice_content),
                notice_status = COALESCE($5, notice_status),
                is_top = COALESCE($6, is_top),
                priority = COALESCE($7, priority),
                publish_time = COALESCE($8, publish_time),
                updated_at = NOW()
            WHERE id = $1
            RETURNING 
                id, notice_title, notice_type, notice_content, notice_status, 
                is_top, priority, publish_time, view_count, publisher_id, 
                publisher_name, created_by, updated_by, created_at, 
                updated_at, deleted_at
            "#,
        )
        .bind(notice_id)
        .bind(notice_title)
        .bind(notice_type)
        .bind(notice_content)
        .bind(notice_status)
        .bind(is_top)
        .bind(priority)
        .bind(publish_time_db)
        .fetch_one(db)
        .await?;
        Ok(n)
    }

    pub async fn delete_notice(db: &PgPool, notice_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE sys_notice 
            SET 
                deleted_at = NOW(),
                updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(notice_id)
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn increment_view_count(
        db: &PgPool,
        notice_id: Uuid,
    ) -> Result<i32, AppError> {
        let row = sqlx::query(
            r#"
            UPDATE sys_notice 
            SET 
                view_count = view_count + 1,
                updated_at = NOW()
            WHERE id = $1
            RETURNING view_count
            "#,
        )
        .bind(notice_id)
        .fetch_one(db)
        .await?;
        let count: i32 = row.get(0);
        Ok(count)
    }
}
