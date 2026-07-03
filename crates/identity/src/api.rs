//! Public API Interfaces

//! Identity Management
pub struct RbacRole { pub name: String, pub permissions: std::vec::Vec<String> }
pub struct AbacPolicy { pub resource: String, pub action: String, pub conditions: String }
