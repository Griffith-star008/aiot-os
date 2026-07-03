/// Enterprise Observability API
use aiot_core::api::AiotError;

pub trait OpenTelemetryExporter {
    fn export_metrics(&self) -> Result<(), AiotError>;
    fn export_traces(&self) -> Result<(), AiotError>;
}

pub trait PrometheusScraper {
    fn get_metrics(&self) -> String;
}

pub trait JaegerTracer {
    fn start_span(&mut self, name: &str);
    fn end_span(&mut self);
}

pub struct ErrorBudget {
    pub total_budget: u32,
    pub consumed: u32,
}

pub struct SLO {
    pub target_percentage: f32,
}
