use std::string::String;

#[derive(Clone, Debug, PartialEq)]
/// Documentation for ModelProvider.
pub enum ModelProvider {
    Local,
    OpenAI,
    Claude,
    Gemini,
    DeepSeek,
}

/// Documentation for RoutingRequest.
pub struct RoutingRequest {
    /// Documentation for field `prompt`.
    pub prompt: String,
    /// Documentation for field `max_tokens`.
    pub max_tokens: u32,
    /// Documentation for field `latency_sensitive`.
    pub latency_sensitive: bool,
    /// Documentation for field `cost_sensitive`.
    pub cost_sensitive: bool,
}

/// Documentation for MultiModelRouter.
pub struct MultiModelRouter {
    /// Documentation for field `default_provider`.
    pub default_provider: ModelProvider,
}

impl MultiModelRouter {
    /// Documentation for new.
    pub fn new() -> Self {
        Self {
            default_provider: ModelProvider::Local,
        }
    }

    /// Documentation for route.
    pub fn route(&self, request: &RoutingRequest) -> ModelProvider {
        if request.latency_sensitive {
            // Local model is fastest
            return ModelProvider::Local;
        }

        if request.cost_sensitive {
            // DeepSeek is cost-effective
            return ModelProvider::DeepSeek;
        }

        if request.max_tokens > 100000 {
            // Claude handles huge contexts well
            return ModelProvider::Claude;
        }

        // Default fallback to powerful models
        ModelProvider::OpenAI
    }
}

impl Default for MultiModelRouter {
    fn default() -> Self {
        Self::new()
    }
}
