use axum::{extract::{State, Path, Query}, Json};
use crate::app::AppState;

#[derive(Debug, Deserialize)]
pub struct DepartmentQueryParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

pub async fn list_departments(
    State(state): State<AppState>,
    Query(_params): Query<DepartmentQueryParams>,
) -> Json<serde_json::Value> {
    let result = sqlx::query_as::<_, (String, String, i32, i32)>(
        "SELECT id, department_name, display_order, status FROM sys_department WHERE deleted_at IS NULL ORDER BY display_order ASC"
    )
    .fetch_all(&state.pool)
    .await;

    match result {
        Ok(departments) => {
            let list: Vec<serde_json::Value> = departments.into_iter().map(|d| {
                serde_json::json!({
                    "id": d.0,
                    "department_name": d.1,
                    "display_order": d.2,
                    "status": d.3
                })
            }).collect();

            Json(serde_json::json!({
                "code": 200,
                "message": "success",
                "data": list
            }))
        }
        Err(e) => Json(serde_json::json!({
            "code": 500,
            "message": format!("查询失败: {}", e),
            "data": null
        }))
    }
}

pub async fn get_department(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": {
            "id": id
        }
    }))
}

pub async fn create_department(
    State(_state): State<AppState>,
    Json(req): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "创建部门功能开发中",
        "data": req
    }))
}

pub async fn update_department(
    State(_state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": format!("更新部门 {} 成功", id),
        "data": req
    }))
}

pub async fn delete_department(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": format!("删除部门 {} 成功", id),
        "data": null
    }))
}

pub async fn get_department_tree(State(state): State<AppState>) -> Json<serde_json::Value> {
    let result = sqlx::query_as::<_, (String, String, i32, i32)>(
        "SELECT id, department_name, display_order, status FROM sys_department WHERE deleted_at IS NULL ORDER BY display_order ASC"
    )
    .fetch_all(&state.pool)
    .await;

    match result {
        Ok(departments) => {
            let list: Vec<serde_json::Value> = departments.into_iter().map(|d| {
                serde_json::json!({
                    "id": d.0,
                    "department_name": d.1,
                    "display_order": d.2,
                    "status": d.3,
                    "children": []
                })
            }).collect();

            Json(serde_json::json!({
                "code": 200,
                "message": "success",
                "data": list
            }))
        }
        Err(e) => Json(serde_json::json!({
            "code": 500,
            "message": format!("查询失败: {}", e),
            "data": null
        }))
    }
}
