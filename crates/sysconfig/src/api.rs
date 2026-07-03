use crate::internal::ConfigReloader;

#[derive(Clone, Debug, PartialEq, Eq)]
/// Documentation for RuntimeConfig.
pub struct RuntimeConfig {
    /// Documentation for field `max_agents`.
    pub max_agents: u32,
    /// Documentation for field `strict_mode`.
    pub strict_mode: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Documentation for SchedulerConfig.
pub struct SchedulerConfig {
    /// Documentation for field `use_rl_scheduler`.
    pub use_rl_scheduler: bool,
    /// Documentation for field `tick_rate_ms`.
    pub tick_rate_ms: u32,
}

/// Documentation for ConfigManager.
pub struct ConfigManager {
    reloader: ConfigReloader,
}

impl ConfigManager {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self {
            reloader: ConfigReloader::new(),
        }
    }

    /// Documentation for reload_config.
    pub fn reload_config(&mut self) -> Result<(), &'static str> {
        self.reloader.hot_reload()
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Documentation for FeatureFlagManager.
pub trait FeatureFlagManager {
    fn is_enabled(&self, feature: &str) -> bool;
    fn enable_feature(&mut self, feature: &str) -> Result<(), &'static str>;
    fn disable_feature(&mut self, feature: &str) -> Result<(), &'static str>;
}

/// Documentation for ConfigMigrator.
pub trait ConfigMigrator {
    fn migrate_to_v2(&mut self) -> Result<(), &'static str>;
    fn migrate_to_v3(&mut self) -> Result<(), &'static str>;
}
