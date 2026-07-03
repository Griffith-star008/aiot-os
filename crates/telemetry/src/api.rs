use std::string::String;
use std::vec::Vec;

/// Documentation for LiveMetrics.
pub struct LiveMetrics {
    /// Documentation for field `cpu_usage`.
    pub cpu_usage: u8,
    /// Documentation for field `memory_usage`.
    pub memory_usage: u8,
    /// Documentation for field `gpu_load`.
    pub gpu_load: u8,
    /// Documentation for field `thermal`.
    pub thermal: i16,
    /// Documentation for field `queue_length`.
    pub queue_length: u32,
    /// Documentation for field `latency_ms`.
    pub latency_ms: u32,
}

#[derive(Clone, Debug)]
/// Documentation for LogRecord.
pub struct LogRecord {
    /// Documentation for field `trace_id`.
    pub trace_id: String,
    /// Documentation for field `request_id`.
    pub request_id: String,
    /// Documentation for field `span_id`.
    pub span_id: String,
    /// Documentation for field `user_id`.
    pub user_id: Option<String>,
    /// Documentation for field `duration_ms`.
    pub duration_ms: u32,
    /// Documentation for field `service_name`.
    pub service_name: String,
    /// Documentation for field `message`.
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum TelemetryError {
    #[error("Export Failed: {0}")]
    ExportFailed(String),
    #[error("Serialization Error: {0}")]
    SerializationError(String),
    #[error("Context Injection Failed")]
    InjectionFailed,
}

impl From<TelemetryError> for aiot_core::api::AiotError {
    fn from(_e: TelemetryError) -> Self {
        aiot_core::api::AiotError::Unknown(0) // Map correctly later
    }
}

/// Documentation for StructuredLogger.
pub trait StructuredLogger {
    fn log_json(&self, record: &LogRecord) -> Result<(), aiot_core::api::AiotError>;
}

/// Documentation for OtelExporter.
pub trait OtelExporter {
    fn export_metrics(&self, metrics: &LiveMetrics) -> Result<(), aiot_core::api::AiotError>;
    fn export_trace(&self, record: &LogRecord) -> Result<(), aiot_core::api::AiotError>;
}

/// Documentation for FlamegraphGenerator.
pub trait FlamegraphGenerator {
    fn generate_flamegraph(&self) -> Vec<u8>;
}

/// Documentation for DistributedTracer.
pub trait DistributedTracer {
    fn start_span(&mut self, name: &str) -> String;
    fn end_span(&mut self, span_id: &str);
    fn inject_context(&self, carrier: &mut [u8]) -> Result<(), aiot_core::api::AiotError>;
    fn extract_context(&self, carrier: &[u8]) -> Result<String, aiot_core::api::AiotError>;
}

/// Telemetry Config
pub struct TelemetryConfig {
    pub endpoint: String,
}

/// Builder for Telemetry subsystem
pub struct TelemetryBuilder {
    endpoint: String,
}

impl TelemetryBuilder {
    pub fn new() -> Self {
        Self {
            endpoint: String::from("http://localhost:4317"),
        }
    }

    pub fn with_endpoint(mut self, endpoint: &str) -> Self {
        self.endpoint = String::from(endpoint);
        self
    }

    pub fn build(self) -> TelemetryConfig {
        TelemetryConfig {
            endpoint: self.endpoint,
        }
    }
}

impl Default for TelemetryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
