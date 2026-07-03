/// Documentation for SafetyInternalLog.
pub struct SafetyInternalLog {
    /// Documentation for field `fault_count`.
    pub fault_count: u32,
}

impl SafetyInternalLog {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self { fault_count: 0 }
    }
}

impl Default for SafetyInternalLog {
    fn default() -> Self {
        Self::new()
    }
}
