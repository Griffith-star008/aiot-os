use std::string::String;
use std::vec::Vec;

/// Documentation for PromptVersion.
pub struct PromptVersion {
    /// Documentation for field `id`.
    pub id: String,
    /// Documentation for field `content`.
    pub content: String,
    /// Documentation for field `version`.
    pub version: String,
}

/// Documentation for ModelVersion.
pub struct ModelVersion {
    /// Documentation for field `id`.
    pub id: String,
    /// Documentation for field `name`.
    pub name: String,
    /// Documentation for field `version`.
    pub version: String,
    /// Documentation for field `endpoint`.
    pub endpoint: String,
}

/// Documentation for PromptRegistry.
pub trait PromptRegistry {
    fn register_prompt(&mut self, prompt: PromptVersion);
    fn get_prompt(&self, id: &str, version: &str) -> Option<&PromptVersion>;
}

/// Documentation for ModelRegistry.
pub trait ModelRegistry {
    fn register_model(&mut self, model: ModelVersion);
    fn get_model(&self, id: &str, version: &str) -> Option<&ModelVersion>;
}

/// Documentation for InMemoryRegistry.
pub struct InMemoryRegistry {
    prompts: Vec<PromptVersion>,
    models: Vec<ModelVersion>,
}

impl InMemoryRegistry {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self {
            prompts: Vec::new(),
            models: Vec::new(),
        }
    }
}

impl Default for InMemoryRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PromptRegistry for InMemoryRegistry {
    fn register_prompt(&mut self, prompt: PromptVersion) {
        self.prompts.push(prompt);
    }

    fn get_prompt(&self, id: &str, version: &str) -> Option<&PromptVersion> {
        self.prompts
            .iter()
            .find(|p| p.id == id && p.version == version)
    }
}

impl ModelRegistry for InMemoryRegistry {
    fn register_model(&mut self, model: ModelVersion) {
        self.models.push(model);
    }

    fn get_model(&self, id: &str, version: &str) -> Option<&ModelVersion> {
        self.models
            .iter()
            .find(|m| m.id == id && m.version == version)
    }
}
