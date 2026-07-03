//! Public API Interfaces

//! AI Runtime
use aiot_core::api::AiotError;

/// Core Inference Backend
pub trait InferenceBackend {
    fn load_model(&mut self, path: &str) -> Result<(), AiotError>;
    fn predict(&self, input: &[u8]) -> Result<Vec<u8>, AiotError>;
}

/// Model Cache
#[derive(Default)]
pub struct ModelCache {
    pub cached_models: std::vec::Vec<String>,
}

/// Model Registry
#[derive(Default)]
pub struct ModelRegistry {
    pub models: std::vec::Vec<String>,
}

/// GPU Scheduler
#[derive(Default)]
pub struct GpuScheduler {
    pub available_memory: u64,
}

pub mod adapters {
    pub struct OnnxRuntime;
    pub struct TensorRt;
    pub struct OpenVino;
    pub struct Cuda;
    pub struct Metal;
    pub struct DirectMl;
    pub struct LlamaCpp;
    pub struct Ollama;
    pub struct OpenAi;
    pub struct Anthropic;
    pub struct Gemini;
}
