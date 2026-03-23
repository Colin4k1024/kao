use crate::db::DbPool;
use crate::error::{AppError, AppResult};
use crate::models::user::{CreateUserRequest, User, UserResponse};
use bcrypt::{hash, verify};

pub struct UserService {
    pool: DbPool,
}

impl UserService {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, name, password, avatar, created_at, updated_at FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: &str) -> AppResult<User> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, name, password, avatar, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(user)
    }

    pub async fn create(&self, req: CreateUserRequest) -> AppResult<UserResponse> {
        // 检查邮箱是否已存在
        if let Some(_) = self.find_by_email(&req.email).await? {
            return Err(AppError::BadRequest("Email already exists".to_string()));
        }

        // 密码哈希
        let hashed_password = hash(&req.password, 10)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let user = User::new(req.email, hashed_password, req.name);

        sqlx::query(
            "INSERT INTO users (id, email, name, password, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(&user.id)
        .bind(&user.email)
        .bind(&user.name)
        .bind(&user.password)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(user.into())
    }

    pub async fn verify_password(&self, email: &str, password: &str) -> AppResult<User> {
        let user = self.find_by_email(email).await?;

        match user {
            Some(u) => {
                if verify(password, &u.password).map_err(|e| AppError::Internal(e.to_string()))? {
                    Ok(u)
                } else {
                    Err(AppError::InvalidPassword)
                }
            }
            None => Err(AppError::Unauthorized("User not found".to_string())),
        }
    }
}
