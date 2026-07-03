/// Documentation for RuntimeInternalMonitor.
pub struct RuntimeInternalMonitor {
    /// Documentation for field `uptime_ticks`.
    pub uptime_ticks: u64,
}

impl RuntimeInternalMonitor {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self { uptime_ticks: 0 }
    }
}

impl Default for RuntimeInternalMonitor {
    fn default() -> Self {
        Self::new()
    }
}
