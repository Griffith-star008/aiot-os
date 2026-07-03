use alloc::boxed::Box;
use std::collections::BTreeMap;
use core::any::{Any, TypeId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for LifecycleState.
pub enum LifecycleState {
    Created,
    Configured,
    Initialized,
    Running,
    Paused,
    Updating,
    Recovering,
    Stopping,
    Stopped,
}

/// Documentation for RuntimeLifecycle.
pub struct RuntimeLifecycle {
    /// Documentation for field `current_state`.
    pub current_state: LifecycleState,
}

impl RuntimeLifecycle {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self {
            current_state: LifecycleState::Created,
        }
    }

    /// Documentation for transition_to.
    pub fn transition_to(&mut self, next_state: LifecycleState) -> Result<(), aiot_core::api::AiotError> {
        self.current_state = next_state;
        Ok(())
    }
}

/// Builder for RuntimeLifecycle
pub struct RuntimeBuilder {
    initial_state: LifecycleState,
}

impl RuntimeBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            initial_state: LifecycleState::Created,
        }
    }

    /// Set initial state
    pub fn with_initial_state(mut self, state: LifecycleState) -> Self {
        self.initial_state = state;
        self
    }

    /// Build the runtime lifecycle
    pub fn build(self) -> RuntimeLifecycle {
        RuntimeLifecycle {
            current_state: self.initial_state,
        }
    }
}

impl Default for RuntimeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Documentation for ServiceRegistry.
pub trait ServiceRegistry {
    fn register<T: 'static>(&mut self, service: T);
    fn resolve<T: 'static>(&self) -> Option<&T>;
}

/// Documentation for DiContainer.
pub struct DiContainer {
    services: BTreeMap<TypeId, Box<dyn Any>>,
}

impl DiContainer {
    /// Documentation for new.
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
        }
    }
}

impl Default for DiContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceRegistry for DiContainer {
    fn register<T: 'static>(&mut self, instance: T) {
        self.services.insert(TypeId::of::<T>(), Box::new(instance));
    }

    fn resolve<T: 'static>(&self) -> Option<&T> {
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref())
    }
}

/// Builder for DiContainer
pub struct ContainerBuilder {
    container: DiContainer,
}

impl ContainerBuilder {
    pub fn new() -> Self {
        Self {
            container: DiContainer::new(),
        }
    }

    pub fn register<T: 'static>(mut self, instance: T) -> Self {
        self.container.register(instance);
        self
    }

    pub fn build(self) -> DiContainer {
        self.container
    }
}

impl Default for ContainerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for RuntimeLifecycle {
    fn default() -> Self {
        Self::new()
    }
}

use aiot_core::api::AiotError;

/// Documentation for ShutdownManager.
pub trait ShutdownManager {
    fn stop_accepting_requests(&mut self) -> Result<(), AiotError>;
    fn drain_queue(&mut self) -> Result<(), AiotError>;
    fn wait_running_tasks(&mut self) -> Result<(), AiotError>;
    fn flush_logs(&mut self) -> Result<(), AiotError>;
    fn flush_metrics(&mut self) -> Result<(), AiotError>;
    fn close_database(&mut self) -> Result<(), AiotError>;
    fn shutdown_runtime(&mut self) -> Result<(), AiotError>;
}

/// Runtime Abstraction Layer
use aiot_core::api::AiotError;
use core::future::Future;
use core::pin::Pin;

pub trait AiotRuntime {
    fn spawn(&self, future: Pin<Box<dyn Future<Output = ()> + Send>>) -> Result<(), AiotError>;
    fn delay(&self, ms: u64) -> Result<(), AiotError>;
}

pub mod adapters {
    pub struct TokioAdapter;
    pub struct AsyncStdAdapter;
    pub struct WasmAdapter;
    pub struct FreeRtosAdapter;
    pub struct ZephyrAdapter;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    Created,
    Starting,
    Running,
    Stopped,
}

pub struct Runtime {
    pub current_state: LifecycleState,
}

impl Runtime {
    pub fn transition_to(&mut self, state: LifecycleState) -> Result<(), AiotError> {
        self.current_state = state;
        Ok(())
    }
}

pub struct RuntimeBuilder {
    state: LifecycleState,
}

impl Default for RuntimeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeBuilder {
    pub fn new() -> Self { Self { state: LifecycleState::Created } }
    pub fn with_initial_state(mut self, state: LifecycleState) -> Self { 
        self.state = state; 
        self 
    }
    pub fn build(self) -> Runtime { 
        Runtime { current_state: self.state }
    }
    pub fn start(&mut self) -> Result<(), AiotError> { Ok(()) }
}
