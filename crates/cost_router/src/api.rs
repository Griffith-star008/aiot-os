/// API for cost_router
use aiot_core::api::AiotError;

pub trait CostRouterInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
