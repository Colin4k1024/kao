use axum::{extract::{State, Path}, Json};
use crate::app::AppState;

pub async fn list_notices() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": []
    }))
}

pub async fn get_notice() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": null
    }))
}

pub async fn create_notice() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "创建成功",
        "data": null
    }))
}

pub async fn update_notice() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "更新成功",
        "data": null
    }))
}

pub async fn delete_notice() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "删除成功",
        "data": null
    }))
}

pub async fn publish_notice() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "发布成功",
        "data": null
    }))
}
