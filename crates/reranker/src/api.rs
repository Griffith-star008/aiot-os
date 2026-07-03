/// API for reranker
use aiot_core::api::AiotError;

pub trait RerankerInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
