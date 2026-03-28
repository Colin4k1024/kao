use sqlx::PgPool;

use crate::common::error::AppError;

use super::model::ConfigRecord;

pub struct ConfigRepository;

impl ConfigRepository {
    pub async fn get_config_by_key(
        db: &PgPool,
        config_key: &str,
    ) -> Result<Option<ConfigRecord>, AppError> {
        let c = sqlx::query_as::<_, ConfigRecord>(
            r#"
            SELECT 
                id, config_name, config_key, config_value, config_type, 
                is_encrypt, status, remark, created_by, updated_by, 
                created_at, updated_at, deleted_at
            FROM sys_config 
            WHERE config_key = $1 AND (deleted_at IS NULL OR deleted_at > NOW())
            "#,
        )
        .bind(config_key)
        .fetch_optional(db)
        .await?;
        Ok(c)
    }

    pub async fn list_configs(
        db: &PgPool,
        config_key: Option<&str>,
        config_type: Option<&str>,
    ) -> Result<Vec<ConfigRecord>, AppError> {
        let mut query = String::from(
            r#"
            SELECT 
                id, config_name, config_key, config_value, config_type, 
                is_encrypt, status, remark, created_by, updated_by, 
                created_at, updated_at, deleted_at
            FROM sys_config 
            WHERE deleted_at IS NULL OR deleted_at > NOW()
            "#,
        );
        
        if let Some(key) = config_key {
            query.push_str(&format!(" AND config_key LIKE '%{}%'", key));
        }
        if let Some(t) = config_type {
            query.push_str(&format!(" AND config_type = '{}'", t));
        }
        query.push_str(" ORDER BY created_at DESC");
        
        let configs = sqlx::query_as::<_, ConfigRecord>(&query)
            .fetch_all(db)
            .await?;
        Ok(configs)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create_config(
        db: &PgPool,
        config_name: String,
        config_key: String,
        config_value: String,
        config_type: Option<String>,
        is_encrypt: Option<String>,
        status: Option<i32>,
        remark: Option<String>,
    ) -> Result<ConfigRecord, AppError> {
        let c = sqlx::query_as::<_, ConfigRecord>(
            r#"
            INSERT INTO sys_config (
                config_name, config_key, config_value, config_type, is_encrypt, 
                status, remark
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING 
                id, config_name, config_key, config_value, config_type, 
                is_encrypt, status, remark, created_by, updated_by, 
                created_at, updated_at, deleted_at
            "#,
        )
        .bind(config_name)
        .bind(config_key)
        .bind(config_value)
        .bind(config_type)
        .bind(is_encrypt)
        .bind(status)
        .bind(remark)
        .fetch_one(db)
        .await?;
        Ok(c)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update_config(
        db: &PgPool,
        config_key: &str,
        config_name: Option<String>,
        new_config_key: Option<String>,
        config_value: Option<String>,
        config_type: Option<String>,
        is_encrypt: Option<String>,
        status: Option<i32>,
        remark: Option<String>,
    ) -> Result<ConfigRecord, AppError> {
        let c = sqlx::query_as::<_, ConfigRecord>(
            r#"
            UPDATE sys_config 
            SET 
                config_name = COALESCE($2, config_name),
                config_key = COALESCE($3, config_key),
                config_value = COALESCE($4, config_value),
                config_type = COALESCE($5, config_type),
                is_encrypt = COALESCE($6, is_encrypt),
                status = COALESCE($7, status),
                remark = COALESCE($8, remark),
                updated_at = NOW()
            WHERE config_key = $1
            RETURNING 
                id, config_name, config_key, config_value, config_type, 
                is_encrypt, status, remark, created_by, updated_by, 
                created_at, updated_at, deleted_at
            "#,
        )
        .bind(config_key)
        .bind(config_name)
        .bind(new_config_key)
        .bind(config_value)
        .bind(config_type)
        .bind(is_encrypt)
        .bind(status)
        .bind(remark)
        .fetch_one(db)
        .await?;
        Ok(c)
    }

    pub async fn delete_config(db: &PgPool, config_key: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE sys_config 
            SET 
                deleted_at = NOW(),
                updated_at = NOW()
            WHERE config_key = $1
            "#,
        )
        .bind(config_key)
        .execute(db)
        .await?;
        Ok(())
    }
}
