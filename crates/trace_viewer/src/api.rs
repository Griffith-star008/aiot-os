/// API for trace_viewer
use aiot_core::api::AiotError;

pub trait TraceViewerInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
