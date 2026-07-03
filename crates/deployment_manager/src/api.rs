/// API for deployment_manager
use aiot_core::api::AiotError;

pub trait DeploymentManagerInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
