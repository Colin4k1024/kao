use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::common::error::AppError;

use super::model::UserRecord;

pub async fn get_user_by_id(db: &PgPool, user_id: Uuid) -> Result<Option<UserRecord>, AppError> {
    let user = sqlx::query_as::<_, UserRecord>(
        r#"
        SELECT 
            id, username, email, display_name, password_hash, dept_id, 
            phone, avatar_url, status, is_super_admin, last_login_at, 
            last_login_ip, created_at, updated_at, deleted_at
        FROM sys_users 
        WHERE id = $1 AND (deleted_at IS NULL OR deleted_at > NOW())
        "#,
    )
    .bind(user_id)
    .fetch_optional(db)
    .await?;

    Ok(user)
}

pub async fn get_user_by_username(db: &PgPool, username: &str) -> Result<Option<UserRecord>, AppError> {
    let user = sqlx::query_as::<_, UserRecord>(
        r#"
        SELECT 
            id, username, email, display_name, password_hash, dept_id, 
            phone, avatar_url, status, is_super_admin, last_login_at, 
            last_login_ip, created_at, updated_at, deleted_at
        FROM sys_users 
        WHERE username = $1 AND (deleted_at IS NULL OR deleted_at > NOW())
        "#,
    )
    .bind(username)
    .fetch_optional(db)
    .await?;

    Ok(user)
}

pub async fn list_users(
    db: &PgPool,
    page: i64,
    page_size: i64,
    _dept_id: Option<Uuid>,
) -> Result<(Vec<UserRecord>, i64), AppError> {
    let offset = (page - 1) * page_size;
    
    let users = sqlx::query_as::<_, UserRecord>(
        r#"
        SELECT 
            id, username, email, display_name, password_hash, dept_id, 
            phone, avatar_url, status, is_super_admin, last_login_at, 
            last_login_ip, created_at, updated_at, deleted_at
        FROM sys_users 
        WHERE deleted_at IS NULL OR deleted_at > NOW()
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(page_size)
    .bind(offset)
    .fetch_all(db)
    .await?;

    let count_row = sqlx::query("SELECT COUNT(*) FROM sys_users WHERE deleted_at IS NULL OR deleted_at > NOW()")
        .fetch_one(db)
        .await?;
    let count: i64 = count_row.get(0);

    Ok((users, count))
}

pub async fn create_user(
    db: &PgPool,
    username: String,
    email: Option<String>,
    display_name: String,
    password_hash: String,
    dept_id: Option<Uuid>,
    phone: Option<String>,
    avatar_url: Option<String>,
) -> Result<UserRecord, AppError> {
    let user = sqlx::query_as::<_, UserRecord>(
        r#"
        INSERT INTO sys_users (
            username, email, display_name, password_hash, dept_id, 
            phone, avatar_url, status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, 'ACTIVE')
        RETURNING 
            id, username, email, display_name, password_hash, dept_id, 
            phone, avatar_url, status, is_super_admin, last_login_at, 
            last_login_ip, created_at, updated_at, deleted_at
        "#,
    )
    .bind(username)
    .bind(email)
    .bind(display_name)
    .bind(password_hash)
    .bind(dept_id)
    .bind(phone)
    .bind(avatar_url)
    .fetch_one(db)
    .await?;

    Ok(user)
}

pub async fn update_user(
    db: &PgPool,
    user_id: Uuid,
    email: Option<String>,
    display_name: Option<String>,
    dept_id: Option<Uuid>,
    phone: Option<String>,
    avatar_url: Option<String>,
    status: Option<String>,
) -> Result<UserRecord, AppError> {
    let user = sqlx::query_as::<_, UserRecord>(
        r#"
        UPDATE sys_users 
        SET 
            email = COALESCE($2, email),
            display_name = COALESCE($3, display_name),
            dept_id = $4,
            phone = COALESCE($5, phone),
            avatar_url = COALESCE($6, avatar_url),
            status = COALESCE($7, status),
            updated_at = NOW()
        WHERE id = $1
        RETURNING 
            id, username, email, display_name, password_hash, dept_id, 
            phone, avatar_url, status, is_super_admin, last_login_at, 
            last_login_ip, created_at, updated_at, deleted_at
        "#,
    )
    .bind(user_id)
    .bind(email)
    .bind(display_name)
    .bind(dept_id)
    .bind(phone)
    .bind(avatar_url)
    .bind(status)
    .fetch_one(db)
    .await?;

    Ok(user)
}

pub async fn update_user_password(
    db: &PgPool,
    user_id: Uuid,
    password_hash: String,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE sys_users 
        SET 
            password_hash = $2,
            password_changed_at = NOW(),
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .bind(password_hash)
    .execute(db)
    .await?;

    Ok(())
}

pub async fn delete_user(db: &PgPool, user_id: Uuid) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE sys_users 
        SET 
            deleted_at = NOW(),
            status = 'DISABLED',
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .execute(db)
    .await?;

    Ok(())
}