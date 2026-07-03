//! Public API Interfaces

//! Service Discovery
use aiot_core::api::AiotError;

pub trait ServiceDiscoveryProvider {
    fn register_service(&mut self, name: &str) -> Result<(), AiotError>;
    fn discover_service(&self, name: &str) -> Result<String, AiotError>;
}

pub mod providers {
    pub struct Consul;
    pub struct Etcd;
    pub struct Dns;
    pub struct MDns;
    pub struct Kubernetes;
    pub struct Static;
}
