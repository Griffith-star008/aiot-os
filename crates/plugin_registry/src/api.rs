//! Public API Interfaces

//! Plugin Registry
/// Manages installed plugins, their manifests, versions, and capabilities.

use plugin_api::{PluginId, PluginType};
use aiot_core::{AiotError, PluginError};
use std::collections::BTreeMap;
use std::string::String;

extern crate alloc;

#[derive(Clone, Debug)]
pub struct PluginManifest {
    pub id: PluginId,
    pub name: String,
    pub version: u16,
    pub plugin_type: PluginType,
}

pub struct PluginRegistry {
    manifests: BTreeMap<u32, PluginManifest>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            manifests: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, manifest: PluginManifest) -> Result<(), AiotError> {
        if self.manifests.contains_key(&manifest.id.0) {
            return Err(AiotError::Plugin(PluginError::LoadFailed));
        }
        self.manifests.insert(manifest.id.0, manifest);
        Ok(())
    }

    pub fn unregister(&mut self, id: PluginId) -> Result<(), AiotError> {
        self.manifests.remove(&id.0);
        Ok(())
    }

    pub fn get_manifest(&self, id: PluginId) -> Option<&PluginManifest> {
        self.manifests.get(&id.0)
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
