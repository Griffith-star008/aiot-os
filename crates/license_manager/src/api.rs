/// API for license_manager
use aiot_core::api::AiotError;

pub trait LicenseManagerInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
