/// Documentation for SecurityInternalContext.
pub struct SecurityInternalContext {
    /// Documentation for field `is_locked`.
    pub is_locked: bool,
}

impl SecurityInternalContext {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self { is_locked: false }
    }
}

impl Default for SecurityInternalContext {
    fn default() -> Self {
        Self::new()
    }
}
