/// Documentation for TelemetryInternalState.
pub struct TelemetryInternalState {
    /// Documentation for field `sample_count`.
    pub sample_count: u64,
}

impl TelemetryInternalState {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self { sample_count: 0 }
    }
}

impl Default for TelemetryInternalState {
    fn default() -> Self {
        Self::new()
    }
}
