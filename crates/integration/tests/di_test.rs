use runtime::{ContainerBuilder, ServiceRegistry};
use scheduler::{AutonomousScheduler, SchedulerBuilder};
use storage::{StorageBuilder, StorageConfig, StorageEngineType};

#[test]
fn test_dependency_injection_container() {
    // Build and configure the storage subsystem
    let storage_config = StorageBuilder::new()
        .with_engine(StorageEngineType::Memory)
        .build();

    // Build and configure the scheduler subsystem
    let scheduler = SchedulerBuilder::new()
        .with_learning_rate(0.05)
        .build();

    // Register all services into the DI container
    let container = ContainerBuilder::new()
        .register::<StorageConfig>(storage_config)
        // Store as Box<dyn TaskScheduler> or AutonomousScheduler directly
        .register::<AutonomousScheduler>(scheduler)
        .build();

    // Resolve storage configuration
    let resolved_storage = container.resolve::<StorageConfig>();
    assert!(resolved_storage.is_some());
    
    // Resolve scheduler
    let resolved_scheduler = container.resolve::<AutonomousScheduler>();
    assert!(resolved_scheduler.is_some());
    assert!((resolved_scheduler.unwrap().learning_rate - 0.05).abs() < f32::EPSILON);
}
