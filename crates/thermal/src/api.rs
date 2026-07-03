#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for ThermalZone.
pub enum ThermalZone {
    Cpu,
    Gpu,
    Battery,
    Ambient,
}

/// Documentation for ThermalSensor.
pub trait ThermalSensor {
    fn read_temperature(&self) -> i16; // In Celsius
}

/// Documentation for ThermalManager.
pub struct ThermalManager {
    /// Documentation for field `max_safe_temp`.
    pub max_safe_temp: i16,
    /// Documentation for field `current_temp`.
    pub current_temp: i16,
}

impl ThermalManager {
    /// Documentation for new.
    pub fn new(max_safe_temp: i16) -> Self {
        Self {
            max_safe_temp,
            current_temp: 25, // Default room temp
        }
    }

    /// Documentation for update_temperature.
    pub fn update_temperature(&mut self, temp: i16) {
        self.current_temp = temp;
    }

    /// Documentation for is_overheating.
    pub fn is_overheating(&self) -> bool {
        self.current_temp > self.max_safe_temp
    }

    /// Documentation for calculate_throttle_percentage.
    pub fn calculate_throttle_percentage(&self) -> u8 {
        if self.current_temp <= self.max_safe_temp {
            return 0;
        }

        let excess = self.current_temp - self.max_safe_temp;
        if excess > 20 {
            100 // Force shutdown or max throttle
        } else {
            (excess * 5) as u8 // Throttle 5% per degree
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermal_manager_normal() {
        let mut manager = ThermalManager::new(80);
        manager.update_temperature(60);
        assert!(!manager.is_overheating());
        assert_eq!(manager.calculate_throttle_percentage(), 0);
    }

    #[test]
    fn test_thermal_manager_overheating() {
        let mut manager = ThermalManager::new(80);
        manager.update_temperature(85);
        assert!(manager.is_overheating());
        assert_eq!(manager.calculate_throttle_percentage(), 25);
    }

    #[test]
    fn test_thermal_manager_critical() {
        let mut manager = ThermalManager::new(80);
        manager.update_temperature(105);
        assert!(manager.is_overheating());
        assert_eq!(manager.calculate_throttle_percentage(), 100);
    }
}
