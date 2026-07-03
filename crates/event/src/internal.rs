/// Documentation for EventInternalBroker.
pub struct EventInternalBroker {
    /// Documentation for field `total_processed`.
    pub total_processed: u64,
    /// Documentation for field `total_dropped`.
    pub total_dropped: u64,
}

impl EventInternalBroker {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self {
            total_processed: 0,
            total_dropped: 0,
        }
    }

    /// Documentation for process.
    pub fn process(&mut self, count: u64) {
        self.total_processed += count;
    }
}

impl Default for EventInternalBroker {
    fn default() -> Self {
        Self::new()
    }
}
