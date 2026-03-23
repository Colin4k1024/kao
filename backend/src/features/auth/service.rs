use bcrypt::verify;
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use crate::common::{
    auth::{claims::Claims, jwt},
    config::AppConfig,
    error::{AppError, AppResult},
    DbPool,
};
use crate::features::menus::service::MenusService;

use super::{
    model::{
        LoginRequest, LoginResponse, PermissionsResponse, ProfileResponse, RoleSummary,
        SessionBundle, SessionSnapshot, UserProfile,
    },
    repo::{AuthRepo, UserSessionRow},
};

const DEFAULT_TOKEN_TTL_SECS: i64 = 7 * 24 * 60 * 60;

#[derive(Clone)]
pub struct AuthService {
    repo: AuthRepo,
    menus_service: MenusService,
}

impl AuthService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            repo: AuthRepo::new(pool.clone()),
            menus_service: MenusService::new(pool),
        }
    }

    pub async fn login(
        &self,
        config: &AppConfig,
        request: LoginRequest,
    ) -> AppResult<LoginResponse> {
        request
            .validate()
            .map_err(|err| AppError::BadRequest(err.to_string()))?;

        let user = self
            .repo
            .find_user_by_username(&request.username)
            .await?
            .ok_or_else(|| AppError::Unauthorized("invalid username or password".to_string()))?;

        self.ensure_login_allowed(&user)?;
        self.verify_password(&request.password, &user.password_hash)?;

        let session = self.load_session(&user.id).await?;
        let session_snapshot = Self::build_snapshot(&user, &session);
        let issued_at = Utc::now().timestamp().max(0) as usize;
        let session_id = Uuid::new_v4().to_string();
        let mut claims = build_claims(
            &SessionSnapshot {
                session_id: Some(session_id.clone()),
                ..session_snapshot
            },
            issued_at,
            DEFAULT_TOKEN_TTL_SECS as usize,
        );
        claims.session_id = Some(session_id);

        let access_token = jwt::encode(&claims, config)?;

        Ok(LoginResponse {
            access_token,
            token_type: "Bearer".to_string(),
            expires_in: DEFAULT_TOKEN_TTL_SECS,
            profile: session.profile,
            permissions: session.permissions,
            menus: session.menus,
        })
    }

    pub async fn profile(&self, user_id: &str) -> AppResult<ProfileResponse> {
        Ok(self.load_session(user_id).await?.profile)
    }

    pub async fn permissions(&self, user_id: &str) -> AppResult<PermissionsResponse> {
        Ok(self.load_session(user_id).await?.permissions)
    }

    async fn load_session(&self, user_id: &str) -> AppResult<SessionBundle> {
        let user = self.repo.find_user_by_id(user_id).await?;
        let roles = self.repo.list_roles_by_user_id(user_id).await?;
        let permissions = self.repo.list_permissions_by_user_id(user_id).await?;
        let menus = self.menus_service.list_menu_tree_by_user_id(user_id).await?;

        let profile = ProfileResponse {
            user: UserProfile::from(&user),
            roles: roles.iter().map(RoleSummary::from).collect(),
        };

        let permissions = PermissionsResponse {
            role_ids: roles.iter().map(|role| role.id.clone()).collect(),
            roles: roles.iter().map(|role| role.code.clone()).collect(),
            permissions,
        };

        Ok(SessionBundle {
            profile,
            permissions,
            menus,
        })
    }

    fn build_snapshot(user: &UserSessionRow, session: &SessionBundle) -> SessionSnapshot {
        SessionSnapshot {
            user_id: user.id.clone(),
            email: user
                .email
                .clone()
                .unwrap_or_else(|| user.username.clone()),
            dept_id: user.dept_id.clone(),
            role_ids: session.permissions.role_ids.clone(),
            roles: session.permissions.roles.clone(),
            permissions: session.permissions.permissions.clone(),
            session_id: None,
        }
    }

    fn ensure_login_allowed(&self, user: &UserSessionRow) -> AppResult<()> {
        match user.status.as_str() {
            "ACTIVE" => Ok(()),
            "DISABLED" => Err(AppError::Forbidden(
                "account is disabled".to_string(),
            )),
            "LOCKED" => Err(AppError::Forbidden("account is locked".to_string())),
            other => Err(AppError::Forbidden(format!(
                "account status `{}` is not allowed to login",
                other
            ))),
        }
    }

    fn verify_password(&self, password: &str, password_hash: &str) -> AppResult<()> {
        let verified = verify(password, password_hash)
            .map_err(|err| AppError::Internal(err.to_string()))?;

        if verified {
            Ok(())
        } else {
            Err(AppError::Unauthorized(
                "invalid username or password".to_string(),
            ))
        }
    }
}

pub fn build_claims(
    snapshot: &SessionSnapshot,
    issued_at: usize,
    ttl_secs: usize,
) -> Claims {
    Claims {
        sub: snapshot.user_id.clone(),
        email: snapshot.email.clone(),
        dept_id: snapshot.dept_id.clone(),
        role_ids: snapshot.role_ids.clone(),
        roles: snapshot.roles.clone(),
        permissions: snapshot.permissions.clone(),
        session_id: snapshot.session_id.clone(),
        exp: issued_at.saturating_add(ttl_secs),
        iat: issued_at,
    }
}
