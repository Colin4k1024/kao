#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataScope {
    All,
    Custom,
    Dept,
    DeptAndChild,
    SelfOnly,
}

impl Default for DataScope {
    fn default() -> Self {
        Self::Dept
    }
}
