use crate::{PersistentTaskQueue, QueuedTask};
use aiot_core::api::AiotError;
use std::string::String;
use std::vec::Vec;

/// A stub for a Disk-backed Write-Ahead Log Queue
pub struct DiskPersistentQueue {
    /// Documentation for field `storage_path`.
    pub storage_path: String,
    memory_buffer: Vec<QueuedTask>,
}

impl DiskPersistentQueue {
    #[must_use]
    /// Documentation for new.
    pub fn new(path: &str) -> Self {
        Self {
            storage_path: String::from(path),
            memory_buffer: Vec::new(),
        }
    }

    fn flush_to_disk(&self) -> Result<(), AiotError> {
        // In a real system, use standard file I/O or no_std compatible flash storage
        Ok(())
    }
}

impl PersistentTaskQueue for DiskPersistentQueue {
    fn enqueue(&mut self, task: QueuedTask) -> Result<(), AiotError> {
        self.memory_buffer.push(task);
        self.flush_to_disk()?;
        Ok(())
    }

    fn dequeue(&mut self) -> Result<Option<QueuedTask>, AiotError> {
        if self.memory_buffer.is_empty() {
            Ok(None)
        } else {
            let task = self.memory_buffer.remove(0);
            self.flush_to_disk()?;
            Ok(Some(task))
        }
    }

    fn ack(&mut self, _task_id: u64) -> Result<(), AiotError> {
        self.flush_to_disk()?;
        Ok(())
    }

    fn recover_unacked(&mut self) -> Result<(), AiotError> {
        // Reload from disk
        Ok(())
    }
}
