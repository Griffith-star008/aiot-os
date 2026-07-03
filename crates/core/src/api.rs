#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for SysId.
pub struct SysId(pub u64);

/// Error Taxonomy: Core Errors
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum CoreError {
    #[error("Out of Memory")]
    OutOfMemory,
    #[error("Timeout")]
    Timeout,
    #[error("Permission Denied")]
    PermissionDenied,
    #[error("Not Found")]
    NotFound,
    #[error("Hardware Fault")]
    HardwareFault,
    #[error("Invalid State")]
    InvalidState,
    #[error("Not Initialized")]
    NotInitialized,
}

/// Error Taxonomy: Runtime Errors
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum RuntimeError {
    #[error("Lifecycle Transition Failed")]
    LifecycleTransitionFailed,
    #[error("Component Crash")]
    ComponentCrash,
    #[error("Resource Exhausted")]
    ResourceExhausted,
    #[error("Isolation Violation")]
    IsolationViolation,
}

/// Error Taxonomy: Plugin Errors
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum PluginError {
    #[error("Load Failed")]
    LoadFailed,
    #[error("Symbol Not Found")]
    SymbolNotFound,
    #[error("Version Mismatch")]
    VersionMismatch,
    #[error("Execution Trap")]
    ExecutionTrap,
}

/// Error Taxonomy: Storage Errors
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum StorageError {
    #[error("IO Error")]
    IoError,
    #[error("Corruption Detected")]
    CorruptionDetected,
    #[error("Capacity Exceeded")]
    CapacityExceeded,
    #[error("Lock Failed")]
    LockFailed,
}

/// Error Taxonomy: Scheduler Errors
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SchedulerError {
    #[error("Queue Full")]
    QueueFull,
    #[error("Task Dropped")]
    TaskDropped,
    #[error("Deadline Missed")]
    DeadlineMissed,
    #[error("Priority Inversion")]
    PriorityInversion,
}

/// Error Taxonomy: Agent Errors
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum AgentError {
    #[error("Hallucination Detected")]
    HallucinationDetected,
    #[error("Tool Execution Failed")]
    ToolExecutionFailed,
    #[error("Memory Limit Reached")]
    MemoryLimitReached,
    #[error("Reasoning Timeout")]
    ReasoningTimeout,
}

/// Error Taxonomy: Network Errors
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum NetworkError {
    #[error("Connection Refused")]
    ConnectionRefused,
    #[error("Connection Reset")]
    ConnectionReset,
    #[error("Unreachable")]
    Unreachable,
    #[error("Protocol Violation")]
    ProtocolViolation,
}

/// Consolidated Error Taxonomy for the framework
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum AiotError {
    #[error("Core Error: {0}")]
    Core(#[from] CoreError),
    #[error("Runtime Error: {0}")]
    Runtime(#[from] RuntimeError),
    #[error("Plugin Error: {0}")]
    Plugin(#[from] PluginError),
    #[error("Storage Error: {0}")]
    Storage(#[from] StorageError),
    #[error("Scheduler Error: {0}")]
    Scheduler(#[from] SchedulerError),
    #[error("Agent Error: {0}")]
    Agent(#[from] AgentError),
    #[error("Network Error: {0}")]
    Network(#[from] NetworkError),
    #[error("Unknown Error code: {0}")]
    Unknown(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for SysWarning.
pub enum SysWarning {
    HighThermal,
    HighLatency,
    ApproachingOom,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for SysEvent.
pub enum SysEvent {
    NodeJoined,
    NodeLeft,
    LeaderElected,
    ModelActivated,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for SysHealth.
pub struct SysHealth {
    /// Documentation for field `aggregated_score`.
    pub aggregated_score: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for SysStatus.
pub enum SysStatus {
    Starting,
    Healthy,
    Degraded,
    Critical,
    Stopped,
}

pub type SysResult<T> = Result<T, AiotError>;
