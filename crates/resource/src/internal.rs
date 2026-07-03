/// Documentation for ResourceInternalState.
pub struct ResourceInternalState {
    /// Documentation for field `current_load`.
    pub current_load: u8,
}

impl ResourceInternalState {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self { current_load: 0 }
    }
}

impl Default for ResourceInternalState {
    fn default() -> Self {
        Self::new()
    }
}
