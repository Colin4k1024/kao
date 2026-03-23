use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub dept_id: Option<String>,
    pub role_ids: Vec<String>,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub session_id: Option<String>,
    pub exp: usize,
    pub iat: usize,
}
