/// API for cluster_manager
use aiot_core::api::AiotError;

pub trait ClusterManagerInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
