/// API for release_manager
use aiot_core::api::AiotError;

pub trait ReleaseManagerInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
