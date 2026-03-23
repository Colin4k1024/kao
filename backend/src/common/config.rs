use std::{env, net::SocketAddr};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub environment: String,
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing required environment variable `{0}`")]
    MissingVar(&'static str),
    #[error("invalid value in `{0}`: {1}")]
    InvalidVar(&'static str, String),
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            environment: optional("APP_ENV", "development"),
            host: optional("HOST", "0.0.0.0"),
            port: required_parse("PORT")?,
            database_url: required("DATABASE_URL")?,
            jwt_secret: required("JWT_SECRET")?,
        })
    }

    pub fn bind_addr(&self) -> Result<SocketAddr, ConfigError> {
        format!("{}:{}", self.host, self.port)
            .parse()
            .map_err(|err: std::net::AddrParseError| {
                ConfigError::InvalidVar("HOST/PORT", err.to_string())
            })
    }
}

fn required(name: &'static str) -> Result<String, ConfigError> {
    env::var(name).map_err(|_| ConfigError::MissingVar(name))
}

fn required_parse<T>(name: &'static str) -> Result<T, ConfigError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let value = required(name)?;
    value
        .parse::<T>()
        .map_err(|err| ConfigError::InvalidVar(name, err.to_string()))
}

fn optional(name: &'static str, default: &'static str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_string())
}
