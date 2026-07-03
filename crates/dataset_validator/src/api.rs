/// API for dataset_validator
use aiot_core::api::AiotError;

pub trait DatasetValidatorInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
