use axum::{extract::State, Json};
use crate::app::AppState;

pub async fn get_system_metrics() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": {
            "cpu_usage": 45.5,
            "memory_total": 16000000000,
            "memory_used": 8000000000,
            "memory_percent": 50.0,
            "disk_total": 500000000000,
            "disk_used": 250000000000,
            "disk_percent": 50.0,
            "uptime": 86400
        }
    }))
}

pub async fn list_online_users() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": []
    }))
}

pub async fn force_logout() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "强制下线成功",
        "data": null
    }))
}

pub async fn list_login_logs() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": []
    }))
}

pub async fn list_oper_logs() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": []
    }))
}
