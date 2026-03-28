use sqlx::PgPool;
use uuid::Uuid;

use crate::common::error::AppError;

use super::model::DataRecord;

pub struct DataRepository;

impl DataRepository {
    pub async fn get_data_by_id(
        db: &PgPool,
        data_id: Uuid,
    ) -> Result<Option<DataRecord>, AppError> {
        let d = sqlx::query_as::<_, DataRecord>(
            r#"
            SELECT 
                id, dict_sort, dict_label, dict_value, dict_type, css_class, 
                list_class, is_default, status, remark, 
                created_by, updated_by, created_at, updated_at, deleted_at
            FROM sys_dict_data 
            WHERE id = $1 AND (deleted_at IS NULL OR deleted_at > NOW())
            "#,
        )
        .bind(data_id)
        .fetch_optional(db)
        .await?;
        Ok(d)
    }

    pub async fn list_data_by_type(
        db: &PgPool,
        dict_type: &str,
    ) -> Result<Vec<DataRecord>, AppError> {
        let data = sqlx::query_as::<_, DataRecord>(
            r#"
            SELECT 
                id, dict_sort, dict_label, dict_value, dict_type, css_class, 
                list_class, is_default, status, remark, 
                created_by, updated_by, created_at, updated_at, deleted_at
            FROM sys_dict_data 
            WHERE dict_type = $1 AND (deleted_at IS NULL OR deleted_at > NOW())
            ORDER BY dict_sort ASC
            "#,
        )
        .bind(dict_type)
        .fetch_all(db)
        .await?;
        Ok(data)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create_data(
        db: &PgPool,
        dict_sort: Option<i32>,
        dict_label: String,
        dict_value: String,
        dict_type: String,
        css_class: Option<String>,
        list_class: Option<String>,
        is_default: Option<String>,
        status: Option<i32>,
        remark: Option<String>,
    ) -> Result<DataRecord, AppError> {
        let d = sqlx::query_as::<_, DataRecord>(
            r#"
            INSERT INTO sys_dict_data (
                dict_sort, dict_label, dict_value, dict_type, css_class, 
                list_class, is_default, status, remark
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING 
                id, dict_sort, dict_label, dict_value, dict_type, css_class, 
                list_class, is_default, status, remark, 
                created_by, updated_by, created_at, updated_at, deleted_at
            "#,
        )
        .bind(dict_sort)
        .bind(dict_label)
        .bind(dict_value)
        .bind(dict_type)
        .bind(css_class)
        .bind(list_class)
        .bind(is_default)
        .bind(status)
        .bind(remark)
        .fetch_one(db)
        .await?;
        Ok(d)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update_data(
        db: &PgPool,
        data_id: Uuid,
        dict_sort: Option<i32>,
        dict_label: Option<String>,
        dict_value: Option<String>,
        dict_type: Option<String>,
        css_class: Option<String>,
        list_class: Option<String>,
        is_default: Option<String>,
        status: Option<i32>,
        remark: Option<String>,
    ) -> Result<DataRecord, AppError> {
        let d = sqlx::query_as::<_, DataRecord>(
            r#"
            UPDATE sys_dict_data 
            SET 
                dict_sort = COALESCE($2, dict_sort),
                dict_label = COALESCE($3, dict_label),
                dict_value = COALESCE($4, dict_value),
                dict_type = COALESCE($5, dict_type),
                css_class = COALESCE($6, css_class),
                list_class = COALESCE($7, list_class),
                is_default = COALESCE($8, is_default),
                status = COALESCE($9, status),
                remark = COALESCE($10, remark),
                updated_at = NOW()
            WHERE id = $1
            RETURNING 
                id, dict_sort, dict_label, dict_value, dict_type, css_class, 
                list_class, is_default, status, remark, 
                created_by, updated_by, created_at, updated_at, deleted_at
            "#,
        )
        .bind(data_id)
        .bind(dict_sort)
        .bind(dict_label)
        .bind(dict_value)
        .bind(dict_type)
        .bind(css_class)
        .bind(list_class)
        .bind(is_default)
        .bind(status)
        .bind(remark)
        .fetch_one(db)
        .await?;
        Ok(d)
    }

    pub async fn delete_data(db: &PgPool, data_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE sys_dict_data 
            SET 
                deleted_at = NOW(),
                updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(data_id)
        .execute(db)
        .await?;
        Ok(())
    }
}
