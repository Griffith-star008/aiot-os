/// Documentation for ProductionInternalData.
pub struct ProductionInternalData {
    /// Documentation for field `active_deployments`.
    pub active_deployments: u32,
}

impl ProductionInternalData {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self {
            active_deployments: 0,
        }
    }
}

impl Default for ProductionInternalData {
    fn default() -> Self {
        Self::new()
    }
}
