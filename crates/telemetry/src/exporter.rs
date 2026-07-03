use crate::api::{LiveMetrics, LogRecord, OtelExporter};
use alloc::format;
use std::string::String;

/// Documentation for JsonOtelExporter.
pub struct JsonOtelExporter {
    /// Documentation for field `endpoint`.
    pub endpoint: String,
}

impl JsonOtelExporter {
    /// Documentation for new.
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: String::from(endpoint),
        }
    }
}

impl OtelExporter for JsonOtelExporter {
    fn export_metrics(&self, metrics: &LiveMetrics) -> Result<(), aiot_core::api::AiotError> {
        let _json_payload = format!(
            r#"{{"cpu":{},"mem":{},"gpu":{},"queue":{},"lat":{}}}"#,
            metrics.cpu_usage,
            metrics.memory_usage,
            metrics.gpu_load,
            metrics.queue_length,
            metrics.latency_ms
        );
        // Pretend to send to endpoint
        Ok(())
    }

    fn export_trace(&self, record: &LogRecord) -> Result<(), aiot_core::api::AiotError> {
        let _json_payload = format!(
            r#"{{"trace_id":"{}","span_id":"{}","service":"{}","duration":{}}}"#,
            record.trace_id, record.span_id, record.service_name, record.duration_ms
        );
        // Pretend to send to endpoint
        Ok(())
    }
}
