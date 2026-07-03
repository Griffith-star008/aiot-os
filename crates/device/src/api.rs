//! Public API Interfaces

//! Device Framework
use aiot_core::api::AiotError;

pub trait DeviceDriver {
    fn init(&mut self) -> Result<(), AiotError>;
    fn read(&self) -> Result<std::vec::Vec<u8>, AiotError>;
    fn write(&mut self, data: &[u8]) -> Result<(), AiotError>;
}

pub trait OtaCapability {
    fn trigger_ota(&mut self, url: &str) -> Result<(), AiotError>;
}

pub trait DiagnosticsCapability {
    fn run_diagnostics(&self) -> Result<String, AiotError>;
}

pub mod protocols {
    pub mod camera {}
    pub mod sensor {}
    pub mod gpio {}
    pub mod i2c {}
    pub mod spi {}
    pub mod uart {}
    pub mod usb {}
    pub mod can {}
    pub mod modbus {}
}
