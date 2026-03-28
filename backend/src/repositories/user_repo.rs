use crate::models::{CreateUserDto, User, UpdateUserDto};
use sqlx::postgres::PgPool;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("User not found")]
    NotFound,
    #[error("User already exists")]
    AlreadyExists,
}

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<User, RepositoryError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password, email, phone, nickname, avatar, status, department_id, role_id, created_at, updated_at FROM sys_user WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(RepositoryError::NotFound)?;

        Ok(user)
    }

    pub async fn find_by_username(&self, username: &str) -> Result<User, RepositoryError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password, email, phone, nickname, avatar, status, department_id, role_id, created_at, updated_at FROM sys_user WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(RepositoryError::NotFound)?;

        Ok(user)
    }

    pub async fn create(&self, dto: CreateUserDto) -> Result<User, RepositoryError> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now();

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO sys_user (id, username, password, email, phone, nickname, status, department_id, role_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, 1, $7, $8, $9, $10)
            RETURNING id, username, password, email, phone, nickname, avatar, status, department_id, role_id, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(&dto.username)
        .bind(&dto.password)
        .bind(dto.email)
        .bind(dto.phone)
        .bind(dto.nickname)
        .bind(dto.department_id)
        .bind(dto.role_id)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update(&self, id: Uuid, dto: UpdateUserDto) -> Result<User, RepositoryError> {
        let now = chrono::Utc::now();

        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE sys_user
            SET email = COALESCE($2, email),
                phone = COALESCE($3, phone),
                nickname = COALESCE($4, nickname),
                avatar = COALESCE($5, avatar),
                department_id = COALESCE($6, department_id),
                role_id = COALESCE($7, role_id),
                updated_at = $8
            WHERE id = $1
            RETURNING id, username, password, email, phone, nickname, avatar, status, department_id, role_id, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(dto.email)
        .bind(dto.phone)
        .bind(dto.nickname)
        .bind(dto.avatar)
        .bind(dto.department_id)
        .bind(dto.role_id)
        .bind(now)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(RepositoryError::NotFound)?;

        Ok(user)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let result = sqlx::query("DELETE FROM sys_user WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }

    pub async fn find_all(&self) -> Result<Vec<User>, RepositoryError> {
        let users = sqlx::query_as::<_, User>(
            "SELECT id, username, password, email, phone, nickname, avatar, status, department_id, role_id, created_at, updated_at FROM sys_user",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }
}
