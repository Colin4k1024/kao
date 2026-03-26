use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user ID as string
    pub username: String,
    pub exp: usize, // Expiration time as UTC timestamp
    pub iat: usize, // Issued at time as UTC timestamp
    pub permissions: Vec<String>,
    pub dept_id: Option<String>, // Department ID as string
    pub roles: Vec<String>,
    #[serde(default)]
    pub token_version: usize, // Token version for revocation
}

impl Claims {
    pub fn new(
        user_id: Uuid,
        username: String,
        permissions: Vec<String>,
        dept_id: Option<Uuid>,
        roles: Vec<String>,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        Self {
            sub: user_id.to_string(),
            username,
            exp: now + 24 * 60 * 60, // Token expires in 24 hours
            iat: now,
            permissions,
            dept_id: dept_id.map(|id| id.to_string()),
            roles,
            token_version: 0, // Initial token version
        }
    }

    /// Create a new claim with updated token version
    pub fn with_version(&self, version: usize) -> Self {
        Claims {
            token_version: version,
            ..self.clone()
        }
    }
}
