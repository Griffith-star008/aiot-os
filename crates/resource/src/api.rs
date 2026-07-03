#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for ResourcePolicy.
pub enum ResourcePolicy {
    LatencyFirst,
    PowerFirst,
    Balanced,
    ThermalSafe,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for HardwareResource.
pub enum HardwareResource {
    CpuCore(u8),
    GpuCore(u8),
    NpuCore(u8),
    MemoryBytes(u64),
    StorageBytes(u64),
    NetworkBandwidth(u32), // Mbps
}

/// Documentation for ResourceRequest.
pub struct ResourceRequest {
    /// Documentation for field `priority`.
    pub priority: u8,
    /// Documentation for field `requested_resource`.
    pub requested_resource: HardwareResource,
}

/// Documentation for UnifiedAllocator.
pub trait UnifiedAllocator {
    fn set_policy(&mut self, policy: ResourcePolicy);
    fn allocate(&mut self, request: ResourceRequest) -> Result<(), &'static str>;
    fn free(&mut self, resource: HardwareResource);
}

/// Documentation for Autoscaler.
pub trait Autoscaler {
    fn evaluate_load(&self) -> u8; // 0 - 100%
    fn expand_resources(&mut self) -> Result<(), &'static str>;
    fn shrink_resources(&mut self);
}
