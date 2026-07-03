/// API for gpu_manager
use aiot_core::api::AiotError;

pub trait GpuManagerInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
