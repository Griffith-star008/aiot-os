//! Public API Interfaces

//! Policy Engine
pub trait PolicyEngine {
    fn evaluate(&self, context: &str) -> bool;
}
