/// API for resource_scheduler
use aiot_core::api::AiotError;

pub trait ResourceSchedulerInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
