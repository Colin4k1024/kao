use crate::common::{AppResult, DbPool};

use super::model::MenuRow;

#[derive(Clone)]
pub struct MenusRepo {
    pool: DbPool,
}

impl MenusRepo {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn list_menu_rows_by_user_id(&self, user_id: &str) -> AppResult<Vec<MenuRow>> {
        let rows = sqlx::query_as::<_, MenuRow>(
            r#"
            SELECT DISTINCT
                m.id::text AS id,
                m.parent_id::text AS parent_id,
                m.name,
                m.menu_type,
                m.route_path,
                m.component,
                m.permission,
                m.icon,
                m.sort_order,
                m.visible,
                m.keep_alive
            FROM sys_menus m
            INNER JOIN sys_role_menus rm ON rm.menu_id = m.id
            INNER JOIN sys_user_roles ur ON ur.role_id = rm.role_id
            WHERE ur.user_id = $1::uuid
            ORDER BY m.sort_order, m.name
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}
