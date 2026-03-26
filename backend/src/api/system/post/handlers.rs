use axum::{extract::{State, Path}, Json};
use crate::app::AppState;

pub async fn list_posts() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": []
    }))
}

pub async fn get_post() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": null
    }))
}

pub async fn create_post() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "创建成功",
        "data": null
    }))
}

pub async fn update_post() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "更新成功",
        "data": null
    }))
}

pub async fn delete_post() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "删除成功",
        "data": null
    }))
}
