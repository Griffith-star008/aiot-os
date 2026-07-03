/// API for compatibility
use aiot_core::api::AiotError;

pub trait CompatibilityInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
