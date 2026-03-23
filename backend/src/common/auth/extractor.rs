use crate::common::{auth::claims::Claims, error::AppError};
use axum::{
    extract::{Extension, FromRequestParts},
    http::request::Parts,
};

pub struct CurrentClaims(pub Claims);

impl<S> FromRequestParts<S> for CurrentClaims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(claims) = Extension::<Claims>::from_request_parts(parts, state)
            .await
            .map_err(|_| AppError::Unauthorized("missing authentication context".to_string()))?;

        Ok(Self(claims))
    }
}
