/// API for dataset
use aiot_core::api::AiotError;

pub trait DatasetInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
