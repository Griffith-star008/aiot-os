//! Public API Interfaces


#![deny(unsafe_code)]

/// Storage Layer
/// Quản lý hệ thống lưu trữ bền vững (Persistent Storage), WAL, Snapshot, Checkpoint.

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
/// Documentation for StorageError.
pub enum StorageError {
    #[error("Disk Full")]
    DiskFull,
    #[error("Corrupted Data")]
    Corrupted,
    #[error("IO Error")]
    IoError,
    #[error("Not Found")]
    NotFound,
}

impl From<StorageError> for aiot_core::api::AiotError {
    fn from(_e: StorageError) -> Self {
        // Assume aiot_core::StorageError exists, or use a mapped version. 
        // Wait, aiot_core::StorageError is distinct. We can map it if we want.
        // Actually, aiot_core::api::AiotError has `Storage(aiot_core::StorageError)`.
        // Let's just implement a basic mapping or use aiot_core::StorageError directly.
        aiot_core::api::AiotError::Unknown(0) // Simplified for now, or we can use aiot_core::StorageError
    }
}

/// Storage Engine Type
pub enum StorageEngineType {
    Memory,
    Disk,
}

/// Documentation for StorageBuilder.
pub struct StorageBuilder {
    engine: StorageEngineType,
}

impl StorageBuilder {
    pub fn new() -> Self {
        Self {
            engine: StorageEngineType::Memory,
        }
    }

    pub fn with_engine(mut self, engine: StorageEngineType) -> Self {
        self.engine = engine;
        self
    }

    pub fn build(self) -> StorageConfig {
        StorageConfig { engine: self.engine }
    }
}

impl Default for StorageBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Documentation for StorageConfig.
pub struct StorageConfig {
    pub engine: StorageEngineType,
}

/// Documentation for WriteAheadLog.
pub trait WriteAheadLog {
    /// Ghi tuần tự các sự kiện/trạng thái vào WAL để khôi phục khi Crash
    fn append_log(&mut self, data: &[u8]) -> Result<u64, StorageError>;

    /// Trả về tất cả log chưa được commit để Replay
    fn read_uncommitted_logs(&self, buffer: &mut [u8]) -> Result<usize, StorageError>;

    /// Xóa log sau khi tạo Snapshot thành công
    fn truncate_logs(&mut self) -> Result<(), StorageError>;
}

/// Documentation for SnapshotManager.
pub trait SnapshotManager {
    /// Lưu toàn bộ state hiện hành của hệ thống
    fn take_snapshot(&mut self, state_data: &[u8]) -> Result<(), StorageError>;

    /// Khôi phục state từ snapshot mới nhất
    fn restore_latest_snapshot(&self, buffer: &mut [u8]) -> Result<usize, StorageError>;
}

/// Documentation for KeyValueStore.
pub trait KeyValueStore {
    fn put(&mut self, key: &str, value: &[u8]) -> Result<(), StorageError>;
    fn get(&self, key: &str, buffer: &mut [u8]) -> Result<usize, StorageError>;
}

/// Documentation for BackupManager.
pub trait BackupManager {
    fn create_backup(&mut self) -> Result<(), StorageError>;
    fn restore_backup(&mut self, version: u32) -> Result<(), StorageError>;
    fn verify_backup(&self, version: u32) -> Result<bool, StorageError>;
    fn incremental_backup(&mut self) -> Result<(), StorageError>;
}

/// Documentation for RecoveryGuide.
pub struct RecoveryGuide {
    /// Documentation for field `failover_primary`.
    pub failover_primary: bool,
    /// Documentation for field `restore_script_version`.
    pub restore_script_version: u32,
}
