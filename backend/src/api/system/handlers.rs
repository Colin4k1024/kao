use crate::utils::Response;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct SystemInfo {
    pub name: String,
    pub version: String,
    pub description: String,
}

pub async fn system_info() -> Response<SystemInfo> {
    Response::success(SystemInfo {
        name: "kao-backend".to_string(),
        version: "0.1.0".to_string(),
        description: "Kao backend API server".to_string(),
    })
}

pub async fn health_check() -> Response<&'static str> {
    Response::success("OK")
}
