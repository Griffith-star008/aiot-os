/// API for deployment
use aiot_core::api::AiotError;

pub trait DeploymentInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
