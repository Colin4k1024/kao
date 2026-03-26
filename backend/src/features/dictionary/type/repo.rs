use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::common::error::AppError;

use super::model::TypeRecord;

pub struct TypeRepository;

impl TypeRepository {
    pub async fn get_type_by_id(
        db: &PgPool,
        type_id: Uuid,
    ) -> Result<Option<TypeRecord>, AppError> {
        let t = sqlx::query_as::<_, TypeRecord>(
            r#"
            SELECT 
                id, dict_name, dict_type, status, remark, 
                created_by, updated_by, created_at, updated_at, deleted_at
            FROM sys_dict_type 
            WHERE id = $1 AND (deleted_at IS NULL OR deleted_at > NOW())
            "#,
        )
        .bind(type_id)
        .fetch_optional(db)
        .await?;
        Ok(t)
    }

    pub async fn list_types(db: &PgPool) -> Result<Vec<TypeRecord>, AppError> {
        let types = sqlx::query_as::<_, TypeRecord>(
            r#"
            SELECT 
                id, dict_name, dict_type, status, remark, 
                created_by, updated_by, created_at, updated_at, deleted_at
            FROM sys_dict_type 
            WHERE deleted_at IS NULL OR deleted_at > NOW()
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(db)
        .await?;
        Ok(types)
    }

    pub async fn create_type(
        db: &PgPool,
        dict_name: String,
        dict_type: String,
        status: Option<i32>,
        remark: Option<String>,
    ) -> Result<TypeRecord, AppError> {
        let t = sqlx::query_as::<_, TypeRecord>(
            r#"
            INSERT INTO sys_dict_type (
                dict_name, dict_type, status, remark
            )
            VALUES ($1, $2, $3, $4)
            RETURNING 
                id, dict_name, dict_type, status, remark, 
                created_by, updated_by, created_at, updated_at, deleted_at
            "#,
        )
        .bind(dict_name)
        .bind(dict_type)
        .bind(status)
        .bind(remark)
        .fetch_one(db)
        .await?;
        Ok(t)
    }

    pub async fn update_type(
        db: &PgPool,
        type_id: Uuid,
        dict_name: Option<String>,
        dict_type: Option<String>,
        status: Option<i32>,
        remark: Option<String>,
    ) -> Result<TypeRecord, AppError> {
        let t = sqlx::query_as::<_, TypeRecord>(
            r#"
            UPDATE sys_dict_type 
            SET 
                dict_name = COALESCE($2, dict_name),
                dict_type = COALESCE($3, dict_type),
                status = COALESCE($4, status),
                remark = COALESCE($5, remark),
                updated_at = NOW()
            WHERE id = $1
            RETURNING 
                id, dict_name, dict_type, status, remark, 
                created_by, updated_by, created_at, updated_at, deleted_at
            "#,
        )
        .bind(type_id)
        .bind(dict_name)
        .bind(dict_type)
        .bind(status)
        .bind(remark)
        .fetch_one(db)
        .await?;
        Ok(t)
    }

    pub async fn delete_type(db: &PgPool, type_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE sys_dict_type 
            SET 
                deleted_at = NOW(),
                updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(type_id)
        .execute(db)
        .await?;
        Ok(())
    }
}
