//! Public API Interfaces

#![allow(unsafe_code)]
/// Hardware Abstraction Layer

/// Reads a register from memory
/// # Safety
/// Address must be valid and aligned.
pub unsafe fn read_register(address: *const u32) -> u32 {
    *address
}
