use axum::{extract::{State, Path}, Json};
use crate::app::AppState;

pub async fn list_roles() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": []
    }))
}

pub async fn get_role() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": null
    }))
}

pub async fn create_role() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "创建成功",
        "data": null
    }))
}

pub async fn update_role() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "更新成功",
        "data": null
    }))
}

pub async fn delete_role() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "删除成功",
        "data": null
    }))
}
