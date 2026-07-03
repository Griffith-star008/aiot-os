/// Documentation for EvolutionStateInternal.
pub struct EvolutionStateInternal {
    /// Documentation for field `last_action`.
    pub last_action: u8,
}

impl EvolutionStateInternal {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self { last_action: 0 }
    }
}

impl Default for EvolutionStateInternal {
    fn default() -> Self {
        Self::new()
    }
}
