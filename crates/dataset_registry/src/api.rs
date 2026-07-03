/// API for dataset_registry
use aiot_core::api::AiotError;

pub trait DatasetRegistryInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
