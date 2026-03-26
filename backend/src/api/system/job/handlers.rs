use axum::{extract::{State, Path}, Json};
use crate::app::AppState;

pub async fn list_jobs() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": []
    }))
}

pub async fn get_job() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": null
    }))
}

pub async fn create_job() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "创建成功",
        "data": null
    }))
}

pub async fn update_job() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "更新成功",
        "data": null
    }))
}

pub async fn delete_job() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "删除成功",
        "data": null
    }))
}

pub async fn run_job_now() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "任务已触发",
        "data": null
    }))
}

pub async fn list_job_logs() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": []
    }))
}
