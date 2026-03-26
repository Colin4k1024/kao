use axum::{extract::{State, Path}, Json};
use crate::app::AppState;

pub async fn list_menus() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": []
    }))
}

pub async fn get_menu() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": null
    }))
}

pub async fn create_menu() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "创建成功",
        "data": null
    }))
}

pub async fn update_menu() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "更新成功",
        "data": null
    }))
}

pub async fn delete_menu() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "删除成功",
        "data": null
    }))
}

pub async fn get_menu_tree() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": []
    }))
}
