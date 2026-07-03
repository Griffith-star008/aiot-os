/// API for runtime_profiler
use aiot_core::api::AiotError;

pub trait RuntimeProfilerInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
