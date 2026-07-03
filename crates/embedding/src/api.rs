/// API for embedding
use aiot_core::api::AiotError;

pub trait EmbeddingInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
