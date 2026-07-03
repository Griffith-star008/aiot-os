use crate::api::ShutdownManager;
use aiot_core::api::AiotError;

/// Documentation for GracefulShutdownController.
pub struct GracefulShutdownController {
    /// Documentation for field `is_shutting_down`.
    pub is_shutting_down: bool,
}

impl GracefulShutdownController {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self {
            is_shutting_down: false,
        }
    }

    /// Documentation for execute_full_shutdown.
    pub fn execute_full_shutdown(&mut self) -> Result<(), AiotError> {
        self.stop_accepting_requests()?;
        self.drain_queue()?;
        self.wait_running_tasks()?;
        self.flush_metrics()?;
        self.flush_logs()?;
        self.close_database()?;
        self.shutdown_runtime()?;
        Ok(())
    }
}

impl Default for GracefulShutdownController {
    fn default() -> Self {
        Self::new()
    }
}

impl ShutdownManager for GracefulShutdownController {
    fn stop_accepting_requests(&mut self) -> Result<(), AiotError> {
        self.is_shutting_down = true;
        Ok(())
    }

    fn drain_queue(&mut self) -> Result<(), AiotError> {
        Ok(())
    }

    fn wait_running_tasks(&mut self) -> Result<(), AiotError> {
        Ok(())
    }

    fn flush_logs(&mut self) -> Result<(), AiotError> {
        Ok(())
    }

    fn flush_metrics(&mut self) -> Result<(), AiotError> {
        Ok(())
    }

    fn close_database(&mut self) -> Result<(), AiotError> {
        Ok(())
    }

    fn shutdown_runtime(&mut self) -> Result<(), AiotError> {
        Ok(())
    }
}
