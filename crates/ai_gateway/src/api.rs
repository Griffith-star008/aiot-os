/// API for ai_gateway
use aiot_core::api::AiotError;

pub trait AiGatewayInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
