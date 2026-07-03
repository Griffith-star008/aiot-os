#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for ComponentType.
pub enum ComponentType {
    Agent,
    Plugin,
    Service,
    Scheduler,
    Model,
    Driver,
}

/// Documentation for RegistryEntry.
pub struct RegistryEntry {
    /// Documentation for field `component_id`.
    pub component_id: u32,
    /// Documentation for field `comp_type`.
    pub comp_type: ComponentType,
    /// Documentation for field `is_active`.
    pub is_active: bool,
}

/// Documentation for CentralRegistry.
pub trait CentralRegistry {
    fn register(&mut self, entry: RegistryEntry) -> Result<(), &'static str>;
    fn unregister(&mut self, id: u32) -> Result<(), &'static str>;
    fn lookup(&self, id: u32) -> Option<&RegistryEntry>;
    fn list_by_type(&self, comp_type: ComponentType) -> u32; // Giả lập đếm số lượng
}
