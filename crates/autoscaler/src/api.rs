/// API for autoscaler
use aiot_core::api::AiotError;

pub trait AutoscalerInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
