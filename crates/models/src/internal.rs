/// Documentation for ModelsInternalCache.
pub struct ModelsInternalCache {
    /// Documentation for field `cached_models`.
    pub cached_models: u32,
}

impl ModelsInternalCache {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self { cached_models: 0 }
    }
}

impl Default for ModelsInternalCache {
    fn default() -> Self {
        Self::new()
    }
}
