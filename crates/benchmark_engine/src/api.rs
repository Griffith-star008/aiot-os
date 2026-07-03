/// API for benchmark_engine
use aiot_core::api::AiotError;

pub trait BenchmarkEngineInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
