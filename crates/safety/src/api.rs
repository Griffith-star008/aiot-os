use std::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for FaultType.
pub enum FaultType {
    MemoryCorruption,
    WatchdogTimeout,
    SensorDisconnect,
    OverVoltage,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for SafetyAction.
pub enum SafetyAction {
    LogWarning,
    RestartComponent(u32),
    SystemReboot,
    Halt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for RiskZone.
pub enum RiskZone {
    Green,
    Yellow,
    Red,
}

impl RiskZone {
    /// Documentation for evaluate.
    pub fn evaluate(fault_count: u32, temp: i16) -> Self {
        if fault_count > 3 || temp > 85 {
            RiskZone::Red
        } else if fault_count > 0 || temp > 70 {
            RiskZone::Yellow
        } else {
            RiskZone::Green
        }
    }
}

/// Documentation for SafetyCritical.
pub trait SafetyCritical {
    fn get_risk_zone(&self) -> RiskZone;
    fn emergency_action(&self) -> SafetyAction;
}

/// Documentation for SafetyMonitor.
pub struct SafetyMonitor {
    active_faults: Vec<FaultType>,
}

impl SafetyMonitor {
    /// Documentation for new.
    pub fn new() -> Self {
        Self {
            active_faults: Vec::new(),
        }
    }

    /// Documentation for report_fault.
    pub fn report_fault(&mut self, fault: FaultType) {
        if !self.active_faults.contains(&fault) {
            self.active_faults.push(fault);
        }
    }

    /// Documentation for clear_fault.
    pub fn clear_fault(&mut self, fault: FaultType) {
        self.active_faults.retain(|&x| x != fault);
    }

    /// Documentation for get_overall_risk.
    pub fn get_overall_risk(&self, temp: i16) -> RiskZone {
        RiskZone::evaluate(self.active_faults.len() as u32, temp)
    }

    /// Documentation for evaluate_system_state.
    pub fn evaluate_system_state(&self) -> SafetyAction {
        if self.active_faults.contains(&FaultType::MemoryCorruption) {
            return SafetyAction::Halt;
        }

        if self.active_faults.contains(&FaultType::WatchdogTimeout) {
            return SafetyAction::SystemReboot;
        }

        if self.active_faults.contains(&FaultType::SensorDisconnect) {
            return SafetyAction::LogWarning;
        }

        SafetyAction::LogWarning
    }
}

impl Default for SafetyMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety_monitor_no_faults() {
        let monitor = SafetyMonitor::new();
        assert_eq!(monitor.evaluate_system_state(), SafetyAction::LogWarning);
    }

    #[test]
    fn test_safety_monitor_memory_corruption() {
        let mut monitor = SafetyMonitor::new();
        monitor.report_fault(FaultType::MemoryCorruption);
        assert_eq!(monitor.evaluate_system_state(), SafetyAction::Halt);
    }

    #[test]
    fn test_safety_monitor_watchdog() {
        let mut monitor = SafetyMonitor::new();
        monitor.report_fault(FaultType::WatchdogTimeout);
        assert_eq!(monitor.evaluate_system_state(), SafetyAction::SystemReboot);
    }

    #[test]
    fn test_risk_zone_evaluation() {
        let mut monitor = SafetyMonitor::new();
        assert_eq!(monitor.get_overall_risk(25), RiskZone::Green);

        monitor.report_fault(FaultType::SensorDisconnect);
        assert_eq!(monitor.get_overall_risk(60), RiskZone::Yellow);

        assert_eq!(monitor.get_overall_risk(90), RiskZone::Red);
    }
}
