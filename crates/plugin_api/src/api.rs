//! Public API Interfaces


#![deny(unsafe_code)]

/// Stable ABI and Traits for AIoT Plugins

extern crate alloc;
use aiot_core::api::AiotError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PluginId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PluginType {
    VisionModel,
    SpeechEngine,
    PlannerHeuristic,
    ControlAlgorithm,
    Custom,
}

pub trait Plugin {
    fn id(&self) -> PluginId;
    fn plugin_type(&self) -> PluginType;
    fn version(&self) -> u16;
    fn start(&mut self) -> Result<(), AiotError>;
    fn stop(&mut self) -> Result<(), AiotError>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbiVersion(pub u16, pub u16);

#[derive(Clone, Debug)]
pub struct PluginSignature(pub std::vec::Vec<u8>);

#[derive(Clone, Debug)]
pub enum PluginPermission {
    NetworkAccess,
    FileSystemAccess,
    AiHardwareAccess,
}

pub trait PluginSandbox {
    fn execute_isolated(&self, task: &dyn Fn() -> Result<(), aiot_core::api::AiotError>) -> Result<(), aiot_core::api::AiotError>;
}
