use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::common::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataScopeContext {
    pub user_id: Uuid,
    pub is_super_admin: bool,
    pub dept_id: Option<Uuid>,
    pub data_scope: DataScope,
    pub accessible_dept_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataScope {
    All,
    Custom,
    Dept,
    DeptAndChild,
    SelfOnly,
}

impl DataScope {
    pub fn from_str(s: &str) -> Self {
        match s {
            "ALL" => DataScope::All,
            "CUSTOM" => DataScope::Custom,
            "DEPT" => DataScope::Dept,
            "DEPT_AND_CHILD" => DataScope::DeptAndChild,
            "SELF" => DataScope::SelfOnly,
            _ => DataScope::Dept,
        }
    }
}

pub async fn build_data_scope_context(
    db: &PgPool,
    user_id: Uuid,
) -> Result<DataScopeContext, AppError> {
    let is_super_admin_row = sqlx::query("SELECT is_super_admin FROM sys_users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(db)
        .await?;
    
    let is_super_admin = is_super_admin_row
        .map(|row| row.get::<bool, _>("is_super_admin"))
        .unwrap_or(false);

    let dept_id_row = sqlx::query("SELECT dept_id FROM sys_users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(db)
        .await?;
    
    let dept_id = dept_id_row
        .map(|row| row.get::<Option<Uuid>, _>("dept_id"))
        .flatten();

    let data_scope_row = sqlx::query(
        r#"
        SELECT sr.data_scope
        FROM sys_user_roles sur
        JOIN sys_roles sr ON sur.role_id = sr.id
        WHERE sur.user_id = $1 AND sr.status = 'ACTIVE'
        ORDER BY 
            CASE sr.data_scope
                WHEN 'SELF' THEN 1
                WHEN 'DEPT' THEN 2
                WHEN 'DEPT_AND_CHILD' THEN 3
                WHEN 'CUSTOM' THEN 4
                WHEN 'ALL' THEN 5
            END
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(db)
    .await?;
    
    let data_scope_str = data_scope_row
        .map(|row| row.get::<String, _>("data_scope"))
        .unwrap_or_else(|| "DEPT".to_string());

    let data_scope = DataScope::from_str(&data_scope_str);

    let accessible_dept_ids: Vec<Uuid> = match (&data_scope, &dept_id) {
        (DataScope::All, _) => {
            let rows = sqlx::query("SELECT id FROM sys_departments")
                .fetch_all(db)
                .await?;
            rows.into_iter()
                .map(|row| row.get::<Uuid, _>(0))
                .collect()
        }
        (DataScope::Custom, _) => {
            let rows = sqlx::query(
                r#"
                SELECT srd.dept_id
                FROM sys_user_roles sur
                JOIN sys_role_departments srd ON sur.role_id = srd.role_id
                WHERE sur.user_id = $1
                "#,
            )
            .bind(user_id)
            .fetch_all(db)
            .await?;
            rows.into_iter()
                .map(|row| row.get::<Uuid, _>(0))
                .collect()
        }
        (DataScope::Dept, Some(ref user_dept)) => {
            vec![*user_dept]
        }
        (DataScope::DeptAndChild, Some(ref user_dept)) => {
            let mut ids = vec![*user_dept];
            let rows = sqlx::query(
                "SELECT id FROM sys_departments WHERE ancestors LIKE '%' || $1::text || '%' OR id = $1::uuid"
            )
            .bind(user_dept.to_string())
            .fetch_all(db)
            .await?;
            let children: Vec<Uuid> = rows.into_iter()
                .map(|row| row.get::<Uuid, _>(0))
                .collect();
            ids.extend(children);
            ids
        }
        (DataScope::SelfOnly, _) => {
            vec![]
        }
        _ => vec![],
    };

    Ok(DataScopeContext {
        user_id,
        is_super_admin,
        dept_id,
        data_scope,
        accessible_dept_ids,
    })
}

impl DataScopeContext {
    pub fn is_all_access(&self) -> bool {
        self.is_super_admin || self.data_scope == DataScope::All
    }

    pub fn filter_user_query(&self, base_query: &str) -> String {
        if self.is_all_access() {
            return base_query.to_string();
        }

        match self.data_scope {
            DataScope::SelfOnly => {
                format!("{} WHERE u.id = '{}'", base_query, self.user_id)
            }
            DataScope::Dept => {
                if let Some(dept_id) = self.dept_id {
                    format!("{} WHERE u.dept_id = '{}'", base_query, dept_id)
                } else {
                    base_query.to_string()
                }
            }
            DataScope::DeptAndChild => {
                if self.accessible_dept_ids.is_empty() {
                    base_query.to_string()
                } else {
                    let dept_list: Vec<String> = self.accessible_dept_ids
                        .iter()
                        .map(|id| format!("'{}'", id))
                        .collect();
                    format!(
                        "{} WHERE u.dept_id IN ({})",
                        base_query,
                        dept_list.join(", ")
                    )
                }
            }
            DataScope::Custom => {
                if self.accessible_dept_ids.is_empty() {
                    base_query.to_string()
                } else {
                    let dept_list: Vec<String> = self.accessible_dept_ids
                        .iter()
                        .map(|id| format!("'{}'", id))
                        .collect();
                    format!(
                        "{} WHERE u.dept_id IN ({})",
                        base_query,
                        dept_list.join(", ")
                    )
                }
            }
            DataScope::All => base_query.to_string(),
        }
    }
}