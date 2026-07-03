/// Documentation for ThermalInternalState.
pub struct ThermalInternalState {
    /// Documentation for field `last_read`.
    pub last_read: u64,
}

impl ThermalInternalState {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self { last_read: 0 }
    }
}

impl Default for ThermalInternalState {
    fn default() -> Self {
        Self::new()
    }
}
