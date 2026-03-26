use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub app: AppSettings,
    pub jwt: JwtSettings,
    pub cors: CorsSettings,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppSettings {
    pub host: String,
    pub port: u16,
    pub env: String,
    pub log_level: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JwtSettings {
    pub secret: String,
    pub access_token_expires_in: u64,
    pub refresh_token_expires_in: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorsSettings {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub max_age: u64,
}

impl Settings {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        Settings {
            database: DatabaseSettings {
                url: env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/kao_db".to_string()),
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
                min_connections: env::var("DATABASE_MIN_CONNECTIONS")
                    .unwrap_or_else(|_| "2".to_string())
                    .parse()
                    .unwrap_or(2),
                connect_timeout: env::var("DATABASE_CONNECT_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30),
                idle_timeout: env::var("DATABASE_IDLE_TIMEOUT")
                    .unwrap_or_else(|_| "600".to_string())
                    .parse()
                    .unwrap_or(600),
            },
            app: AppSettings {
                host: env::var("APP_HOST")
                    .unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("APP_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .unwrap_or(8080),
                env: env::var("APP_ENV")
                    .unwrap_or_else(|_| "development".to_string()),
                log_level: env::var("RUST_LOG")
                    .unwrap_or_else(|_| "info".to_string()),
            },
            jwt: JwtSettings {
                secret: env::var("JWT_SECRET")
                    .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string()),
                access_token_expires_in: env::var("JWT_ACCESS_TOKEN_EXPIRES_IN")
                    .unwrap_or_else(|_| "3600".to_string())
                    .parse()
                    .unwrap_or(3600),
                refresh_token_expires_in: env::var("JWT_REFRESH_TOKEN_EXPIRES_IN")
                    .unwrap_or_else(|_| "604800".to_string())
                    .parse()
                    .unwrap_or(604800),
            },
            cors: CorsSettings {
                allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                    .unwrap_or_else(|_| "*".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
                allowed_methods: env::var("CORS_ALLOWED_METHODS")
                    .unwrap_or_else(|_| "GET,POST,PUT,DELETE,PATCH".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
                allowed_headers: env::var("CORS_ALLOWED_HEADERS")
                    .unwrap_or_else(|_| "Content-Type,Authorization".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
                max_age: env::var("CORS_MAX_AGE")
                    .unwrap_or_else(|_| "3600".to_string())
                    .parse()
                    .unwrap_or(3600),
            },
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
