/// API for evaluation
use aiot_core::api::AiotError;

pub trait EvaluationInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
