use ai_coding_backend::features::{
    departments::{model::DepartmentRow, service::build_department_tree},
    roles::model::CreateRoleRequest,
    users::model::CreateUserRequest,
};
use validator::Validate;

#[test]
fn create_user_request_validation_rejects_missing_role_ids() {
    let request = CreateUserRequest {
        username: "admin".to_string(),
        email: Some("admin@example.com".to_string()),
        display_name: "系统管理员".to_string(),
        password: "Admin123!".to_string(),
        dept_id: Some("dept-1".to_string()),
        avatar_url: None,
        phone: None,
        status: None,
        role_ids: Vec::new(),
    };

    assert!(request.validate().is_err());
}

#[test]
fn create_role_request_validation_rejects_short_code() {
    let request = CreateRoleRequest {
        code: "A".to_string(),
        name: "测试角色".to_string(),
        description: None,
        data_scope: "DEPT".to_string(),
        status: None,
        menu_ids: vec![],
        department_ids: vec![],
    };

    assert!(request.validate().is_err());
}

#[test]
fn build_department_tree_groups_children_by_parent() {
    let rows = vec![
        DepartmentRow {
            id: "root".to_string(),
            code: "ROOT".to_string(),
            name: "总部".to_string(),
            parent_id: None,
            ancestors: String::new(),
            path: "/总部".to_string(),
            sort_order: 0,
            leader: Some("系统管理员".to_string()),
            phone: None,
            email: None,
            status: "ACTIVE".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        DepartmentRow {
            id: "child".to_string(),
            code: "CHILD".to_string(),
            name: "华东大区".to_string(),
            parent_id: Some("root".to_string()),
            ancestors: "root".to_string(),
            path: "/总部/华东大区".to_string(),
            sort_order: 10,
            leader: Some("区域负责人".to_string()),
            phone: None,
            email: None,
            status: "ACTIVE".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
    ];

    let tree = build_department_tree(rows);

    assert_eq!(tree.len(), 1);
    assert_eq!(tree[0].id, "root");
    assert_eq!(tree[0].children.len(), 1);
    assert_eq!(tree[0].children[0].id, "child");
}
