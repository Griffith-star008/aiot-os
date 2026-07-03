/// API for cost_analyzer
use aiot_core::api::AiotError;

pub trait CostAnalyzerInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
