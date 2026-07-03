/// Documentation for SimulationInternalEngine.
pub struct SimulationInternalEngine {
    /// Documentation for field `tick_rate_ms`.
    pub tick_rate_ms: u32,
}

impl SimulationInternalEngine {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self { tick_rate_ms: 10 }
    }
}

impl Default for SimulationInternalEngine {
    fn default() -> Self {
        Self::new()
    }
}
