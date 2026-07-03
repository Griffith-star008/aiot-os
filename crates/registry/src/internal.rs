/// Documentation for RegistryInternalDB.
pub struct RegistryInternalDB {
    /// Documentation for field `total_components`.
    pub total_components: u32,
}

impl RegistryInternalDB {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self {
            total_components: 0,
        }
    }
}

impl Default for RegistryInternalDB {
    fn default() -> Self {
        Self::new()
    }
}
