use bcrypt::hash;
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use crate::common::{
    auth::claims::Claims,
    error::{AppError, AppResult},
    DbPool,
};

use super::{
    model::{
        CreateUserRequest, UpdateUserRequest, UserListItem, UserResponse, UsersResponse,
    },
    repo::{UserRow, UsersRepo},
};

const BCRYPT_COST: u32 = 10;

#[derive(Clone)]
pub struct UsersService {
    repo: UsersRepo,
}

impl UsersService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            repo: UsersRepo::new(pool),
        }
    }

    pub async fn list_users(&self, claims: &Claims) -> AppResult<UsersResponse> {
        self.require_access(claims, "system:user:list")?;

        let rows = self.repo.list_users().await?;
        let mut users = Vec::with_capacity(rows.len());

        for row in rows {
            users.push(self.build_user_item(row).await?);
        }

        Ok(UsersResponse { users })
    }

    pub async fn create_user(
        &self,
        claims: &Claims,
        request: CreateUserRequest,
    ) -> AppResult<UserResponse> {
        self.require_access(claims, "system:user:add")?;
        request
            .validate()
            .map_err(|err| AppError::BadRequest(err.to_string()))?;

        let status = normalize_user_status(request.status.as_deref())?;
        let password_hash = hash(&request.password, BCRYPT_COST)
            .map_err(|err| AppError::Internal(err.to_string()))?;
        let user_id = Uuid::new_v4().to_string();

        self.repo
            .create_user(
                &user_id,
                &request.username,
                &request.email,
                &request.display_name,
                &password_hash,
                request.dept_id.as_deref(),
                request.avatar_url.as_deref(),
                request.phone.as_deref(),
                &status,
            )
            .await?;
        self.repo
            .replace_user_roles(&user_id, &request.role_ids)
            .await?;

        Ok(UserResponse {
            user: self.load_user_item(&user_id).await?,
        })
    }

    pub async fn update_user(
        &self,
        claims: &Claims,
        user_id: &str,
        request: UpdateUserRequest,
    ) -> AppResult<UserResponse> {
        self.require_access(claims, "system:user:edit")?;
        request
            .validate()
            .map_err(|err| AppError::BadRequest(err.to_string()))?;

        let status = normalize_user_status(request.status.as_deref())?;

        self.repo
            .update_user(
                user_id,
                &request.username,
                &request.email,
                &request.display_name,
                request.dept_id.as_deref(),
                request.avatar_url.as_deref(),
                request.phone.as_deref(),
                &status,
            )
            .await?;
        self.repo
            .replace_user_roles(user_id, &request.role_ids)
            .await?;

        Ok(UserResponse {
            user: self.load_user_item(user_id).await?,
        })
    }

    async fn load_user_item(&self, user_id: &str) -> AppResult<UserListItem> {
        let row = self.repo.find_user_by_id(user_id).await?;
        self.build_user_item(row).await
    }

    async fn build_user_item(&self, row: UserRow) -> AppResult<UserListItem> {
        let roles = self.repo.list_roles_by_user_id(&row.id).await?;
        Ok(UserListItem {
            id: row.id,
            username: row.username,
            email: row.email,
            display_name: row.display_name,
            avatar_url: row.avatar_url,
            phone: row.phone,
            dept_id: row.dept_id,
            dept_name: row.dept_name,
            status: row.status,
            is_super_admin: row.is_super_admin,
            role_ids: roles.iter().map(|role| role.role_id.clone()).collect(),
            role_codes: roles.iter().map(|role| role.role_code.clone()).collect(),
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }

    fn require_access(&self, claims: &Claims, permission: &str) -> AppResult<()> {
        if is_super_admin(claims) || claims.permissions.iter().any(|item| item == permission) {
            return Ok(());
        }

        Err(AppError::Forbidden(format!(
            "missing permission `{}`",
            permission
        )))
    }
}

fn normalize_user_status(status: Option<&str>) -> AppResult<String> {
    let normalized = status.unwrap_or("ACTIVE").to_uppercase();

    match normalized.as_str() {
        "ACTIVE" | "DISABLED" | "LOCKED" => Ok(normalized),
        other => Err(AppError::BadRequest(format!(
            "invalid user status `{}`",
            other
        ))),
    }
}

fn is_super_admin(claims: &Claims) -> bool {
    claims.roles.iter().any(|role| role == "SUPER_ADMIN")
}

