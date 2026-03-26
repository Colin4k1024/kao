use crate::models::{CreateUserDto, LoginDto, LoginResponse, User, UserInfo};
use crate::repositories::UserRepository;
use crate::utils::jwt::{create_token, verify_token, JwtError};
use bcrypt::{hash, verify, DEFAULT_COST};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("User not found")]
    UserNotFound,
    #[error("Token error: {0}")]
    TokenError(#[from] JwtError),
    #[error("Password hashing error")]
    HashingError,
    #[error("User already exists")]
    AlreadyExists,
    #[error("Database error: {0}")]
    DatabaseError(#[from] crate::repositories::user_repo::RepositoryError),
}

pub struct AuthService {
    user_repo: UserRepository,
    jwt_secret: String,
    jwt_expires_in: String,
}

impl AuthService {
    pub fn new(user_repo: UserRepository, jwt_secret: String, jwt_expires_in: String) -> Self {
        Self {
            user_repo,
            jwt_secret,
            jwt_expires_in,
        }
    }

    pub async fn login(&self, dto: LoginDto) -> Result<LoginResponse, AuthError> {
        let user = self
            .user_repo
            .find_by_username(&dto.username)
            .await
            .map_err(|_| AuthError::InvalidCredentials)?;

        let is_valid = verify(&dto.password, &user.password).map_err(|_| AuthError::InvalidCredentials)?;

        if !is_valid {
            return Err(AuthError::InvalidCredentials);
        }

        let token = create_token(
            &user.id.to_string(),
            &user.username,
            &self.jwt_secret,
            &self.jwt_expires_in,
        )?;

        let user_info: UserInfo = user.into();

        Ok(LoginResponse {
            token,
            user: user_info,
        })
    }

    pub async fn register(&self, dto: CreateUserDto) -> Result<User, AuthError> {
        let hashed_password = hash(&dto.password, DEFAULT_COST).map_err(|_| AuthError::HashingError)?;

        let user_dto = CreateUserDto {
            password: hashed_password,
            ..dto
        };

        let user = self.user_repo.create(user_dto).await?;

        Ok(user)
    }

    pub fn verify_token(&self, token: &str) -> Result<String, AuthError> {
        let claims = verify_token(token, &self.jwt_secret)?;
        Ok(claims.sub)
    }
}
