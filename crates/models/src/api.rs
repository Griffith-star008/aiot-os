#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for ModelVersion.
pub struct ModelVersion {
    /// Documentation for field `major`.
    pub major: u8,
    /// Documentation for field `minor`.
    pub minor: u8,
    /// Documentation for field `patch`.
    pub patch: u8,
}

#[derive(Clone, Copy)]
/// Documentation for ModelSignature.
pub struct ModelSignature {
    /// Documentation for field `hash`.
    pub hash: [u8; 32],
}

#[derive(Clone, Copy)]
/// Documentation for ModelMetadata.
pub struct ModelMetadata {
    /// Documentation for field `model_id`.
    pub model_id: u32,
    /// Documentation for field `version`.
    pub version: ModelVersion,
    /// Documentation for field `signature`.
    pub signature: ModelSignature,
}

/// Documentation for ModelRegistry.
pub trait ModelRegistry {
    fn register_model(&mut self, meta: ModelMetadata) -> Result<(), &'static str>;
    fn verify_signature(&self, id: u32, hash: &[u8; 32]) -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for LifecycleState.
pub enum LifecycleState {
    Install,
    Validate,
    Shadow,
    Activate,
    Rollback,
}

/// Documentation for ModelLifecycle.
pub trait ModelLifecycle {
    fn transition_state(&mut self, id: u32, next_state: LifecycleState)
        -> Result<(), &'static str>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for ModelDomain.
pub enum ModelDomain {
    Vision,
    Speech,
    LLM,
    Navigation,
}

/// Documentation for MultiModelCoordinator.
pub trait MultiModelCoordinator {
    fn route_inference(&self, domain: ModelDomain, payload: &[u8]) -> Result<(), &'static str>;
    fn aggregate_results(&self) -> [u8; 128];
}

/// Documentation for GpuScheduler.
pub trait GpuScheduler {
    fn monitor_vram(&self) -> u64;
    fn monitor_gpu_load(&self) -> u8;
    fn queue_inference_task(&mut self, payload: &[u8]) -> Result<(), &'static str>;
}

/// Documentation for PromptVersioning.
pub trait PromptVersioning {
    fn get_prompt(&self, prompt_id: &str, version: &str) -> Option<&'static str>;
    fn update_prompt(
        &mut self,
        prompt_id: &str,
        version: &str,
        content: &'static str,
    ) -> Result<(), &'static str>;
}
