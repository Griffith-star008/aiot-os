/// API for resource_manager
use aiot_core::api::AiotError;

pub trait ResourceManagerInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
