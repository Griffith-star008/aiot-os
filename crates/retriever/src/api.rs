/// API for retriever
use aiot_core::api::AiotError;

pub trait RetrieverInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
