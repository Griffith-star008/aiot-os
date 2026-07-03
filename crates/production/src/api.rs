#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for DeploymentStrategy.
pub enum DeploymentStrategy {
    BlueGreen,
    Canary,
    RollingRestart,
}

/// Documentation for OtaManager.
pub trait OtaManager {
    fn verify_firmware(&self, binary: &[u8]) -> bool;
    fn deploy_firmware(
        &mut self,
        binary: &[u8],
        strategy: DeploymentStrategy,
    ) -> Result<(), &'static str>;
    fn rollback(&mut self);
}

/// Documentation for DeviceIdentity.
pub struct DeviceIdentity {
    /// Documentation for field `uuid`.
    pub uuid: [u8; 16],
    /// Documentation for field `current_version`.
    pub current_version: u32,
}

/// Documentation for FleetManager.
pub trait FleetManager {
    fn register_device(&mut self, device: DeviceIdentity);
    fn broadcast_update(&mut self, target_version: u32);
    fn monitor_fleet_health(&self) -> u8; // 0 - 100
}
