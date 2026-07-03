/// Module Registry Client API
use aiot_core::api::AiotError;

pub trait RegistryClient {
    fn fetch_metadata(&self, pkg_name: &str) -> Result<String, AiotError>;
    fn download_tarball(&self, pkg_name: &str, version: &str) -> Result<std::vec::Vec<u8>, AiotError>;
    fn upload_package(&mut self, pkg_bytes: &[u8]) -> Result<(), AiotError>;
    fn authenticate(&mut self, token: &str) -> Result<(), AiotError>;
}

/// Dependency Resolver
pub trait DependencyResolver {
    fn resolve(&self, dependencies: &std::collections::BTreeMap<String, String>) -> Result<(), AiotError>;
}
