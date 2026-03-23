use super::data_scope::DataScope;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DataScopeContext {
    pub user_id: Option<Uuid>,
    pub dept_id: Option<Uuid>,
    pub data_scope: DataScope,
    pub allowed_dept_ids: Vec<Uuid>,
}

impl DataScopeContext {
    pub fn new(data_scope: DataScope) -> Self {
        Self {
            user_id: None,
            dept_id: None,
            data_scope,
            allowed_dept_ids: Vec::new(),
        }
    }
}
