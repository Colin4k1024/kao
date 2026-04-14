use sqlx::PgPool;
use uuid::Uuid;

use crate::common::error::AppError;

use super::model::PostRecord;

pub async fn get_post_by_id(db: &PgPool, post_id: Uuid) -> Result<Option<PostRecord>, AppError> {
    let post = sqlx::query_as::<_, PostRecord>(
        r#"
        SELECT
            id, post_name, post_code, display_order, status,
            created_by, created_at, updated_at
        FROM sys_post
        WHERE id = $1
        "#,
    )
    .bind(post_id)
    .fetch_optional(db)
    .await?;

    Ok(post)
}

pub async fn get_post_by_code(db: &PgPool, post_code: &str) -> Result<Option<PostRecord>, AppError> {
    let post = sqlx::query_as::<_, PostRecord>(
        r#"
        SELECT
            id, post_name, post_code, display_order, status,
            created_by, created_at, updated_at
        FROM sys_post
        WHERE post_code = $1
        "#,
    )
    .bind(post_code)
    .fetch_optional(db)
    .await?;

    Ok(post)
}

pub async fn list_posts(db: &PgPool) -> Result<Vec<PostRecord>, AppError> {
    let posts = sqlx::query_as::<_, PostRecord>(
        r#"
        SELECT
            id, post_name, post_code, display_order, status,
            created_by, created_at, updated_at
        FROM sys_post
        ORDER BY display_order ASC, created_at DESC
        "#,
    )
    .fetch_all(db)
    .await?;

    Ok(posts)
}

pub async fn create_post(
    db: &PgPool,
    post_name: String,
    post_code: String,
    display_order: i32,
    status: i32,
    created_by: Option<String>,
) -> Result<PostRecord, AppError> {
    let post = sqlx::query_as::<_, PostRecord>(
        r#"
        INSERT INTO sys_post (post_name, post_code, display_order, status, created_by)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING
            id, post_name, post_code, display_order, status,
            created_by, created_at, updated_at
        "#,
    )
    .bind(post_name)
    .bind(post_code)
    .bind(display_order)
    .bind(status)
    .bind(created_by)
    .fetch_one(db)
    .await?;

    Ok(post)
}

pub async fn update_post(
    db: &PgPool,
    post_id: Uuid,
    post_name: Option<String>,
    post_code: Option<String>,
    display_order: Option<i32>,
    status: Option<i32>,
) -> Result<PostRecord, AppError> {
    let post = sqlx::query_as::<_, PostRecord>(
        r#"
        UPDATE sys_post
        SET
            post_name = COALESCE($2, post_name),
            post_code = COALESCE($3, post_code),
            display_order = COALESCE($4, display_order),
            status = COALESCE($5, status),
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id, post_name, post_code, display_order, status,
            created_by, created_at, updated_at
        "#,
    )
    .bind(post_id)
    .bind(post_name)
    .bind(post_code)
    .bind(display_order)
    .bind(status)
    .fetch_one(db)
    .await?;

    Ok(post)
}

pub async fn delete_post(db: &PgPool, post_id: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM sys_post WHERE id = $1")
        .bind(post_id)
        .execute(db)
        .await?;

    Ok(())
}
