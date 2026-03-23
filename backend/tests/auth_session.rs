use ai_coding_backend::common::{config::AppConfig, auth::jwt};
use ai_coding_backend::features::auth::{
    model::{LoginRequest, SessionSnapshot},
    service::build_claims,
};
use ai_coding_backend::features::menus::{
    model::MenuRow,
    service::build_menu_tree,
};
use chrono::Utc;
use validator::Validate;

#[test]
fn login_request_validation_rejects_short_password() {
    let request = LoginRequest {
        username: "admin".to_string(),
        password: "12345".to_string(),
    };

    assert!(request.validate().is_err());
}

#[test]
fn build_claims_preserves_session_context() {
    let snapshot = SessionSnapshot {
        user_id: "user-1".to_string(),
        email: "admin@example.com".to_string(),
        dept_id: Some("dept-1".to_string()),
        role_ids: vec!["role-1".to_string()],
        roles: vec!["SUPER_ADMIN".to_string()],
        permissions: vec!["system:user:list".to_string()],
        session_id: Some("session-1".to_string()),
    };

    let claims = build_claims(&snapshot, 1_700_000_000, 60);

    assert_eq!(claims.sub, "user-1");
    assert_eq!(claims.email, "admin@example.com");
    assert_eq!(claims.dept_id.as_deref(), Some("dept-1"));
    assert_eq!(claims.role_ids, vec!["role-1"]);
    assert_eq!(claims.roles, vec!["SUPER_ADMIN"]);
    assert_eq!(claims.permissions, vec!["system:user:list"]);
    assert_eq!(claims.session_id.as_deref(), Some("session-1"));
    assert_eq!(claims.iat, 1_700_000_000);
    assert_eq!(claims.exp, 1_700_000_060);
}

#[test]
fn jwt_roundtrip_preserves_claims() {
    let config = AppConfig {
        environment: "test".to_string(),
        host: "127.0.0.1".to_string(),
        port: 3001,
        database_url: "postgresql://localhost/test".to_string(),
        jwt_secret: "test-secret".to_string(),
    };

    let snapshot = SessionSnapshot {
        user_id: "user-1".to_string(),
        email: "admin@example.com".to_string(),
        dept_id: None,
        role_ids: vec!["role-1".to_string()],
        roles: vec!["SUPER_ADMIN".to_string()],
        permissions: vec!["system:user:list".to_string()],
        session_id: None,
    };

    let issued_at = Utc::now().timestamp().max(0) as usize;
    let claims = build_claims(&snapshot, issued_at, 3600);
    let token = jwt::encode(&claims, &config).expect("token should encode");
    let decoded = jwt::decode(&token, &config).expect("token should decode");

    assert_eq!(decoded.sub, claims.sub);
    assert_eq!(decoded.permissions, claims.permissions);
}

#[test]
fn build_menu_tree_groups_children_under_their_parent() {
    let rows = vec![
        MenuRow {
            id: "root".to_string(),
            parent_id: None,
            name: "系统管理".to_string(),
            menu_type: "DIRECTORY".to_string(),
            route_path: Some("/system".to_string()),
            component: None,
            permission: None,
            icon: Some("settings".to_string()),
            sort_order: 1,
            visible: true,
            keep_alive: false,
        },
        MenuRow {
            id: "child".to_string(),
            parent_id: Some("root".to_string()),
            name: "用户管理".to_string(),
            menu_type: "MENU".to_string(),
            route_path: Some("/system/users".to_string()),
            component: Some("system/users".to_string()),
            permission: Some("system:user:list".to_string()),
            icon: Some("users".to_string()),
            sort_order: 2,
            visible: true,
            keep_alive: true,
        },
    ];

    let tree = build_menu_tree(rows);

    assert_eq!(tree.len(), 1);
    assert_eq!(tree[0].id, "root");
    assert_eq!(tree[0].children.len(), 1);
    assert_eq!(tree[0].children[0].id, "child");
}
