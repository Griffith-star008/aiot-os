use plugins::{Plugin, PluginId, PluginType};
use runtime::{RuntimeBuilder, LifecycleState};
use aiot_core::AiotError;
use std::thread;
use std::time::Duration;

struct DummySensorPlugin {
    id: PluginId,
}

impl Plugin for DummySensorPlugin {
    fn id(&self) -> PluginId {
        PluginId(self.id.0)
    }
    
    fn plugin_type(&self) -> PluginType {
        PluginType::ControlAlgorithm
    }

    fn version(&self) -> u16 {
        1
    }

    fn start(&mut self) -> Result<(), AiotError> {
        println!("[SensorPlugin] Starting sensor...");
        Ok(())
    }

    fn stop(&mut self) -> Result<(), AiotError> {
        println!("[SensorPlugin] Stopping sensor...");
        Ok(())
    }
}

fn main() {
    println!("Welcome to AIoT Framework: Sensor Demo Example");
    
    let mut runtime = RuntimeBuilder::new()
        .with_initial_state(LifecycleState::Created)
        .build();
        
    runtime.transition_to(LifecycleState::Running).unwrap();

    let mut plugin = DummySensorPlugin {
        id: PluginId(1),
    };

    plugin.start().unwrap();

    println!("Simulating data reading...");
    for i in 1..=3 {
        println!("[SensorPlugin] Reading temperature: {} C", 25 + i);
        thread::sleep(Duration::from_millis(500));
    }

    plugin.stop().unwrap();
    println!("Demo finished.");
}
