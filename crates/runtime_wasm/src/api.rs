//! Public API Interfaces

//! WASM Runtime Sandbox
pub trait WasmSandbox {
    fn instantiate(&mut self, module: &[u8]) -> Result<(), aiot_core::api::AiotError>;
    fn call_function(&self, name: &str, args: &[u8]) -> Result<std::vec::Vec<u8>, aiot_core::api::AiotError>;
}
