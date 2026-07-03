#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for FederatedError.
pub enum FederatedError {
    TrainingDiverged,
    NetworkTimeout,
    AggregationFailed,
}

/// Documentation for ModelGradients.
pub struct ModelGradients {
    /// Documentation for field `payload`.
    pub payload: [u8; 1024],
}

/// Documentation for FederatedNode.
pub trait FederatedNode {
    fn train_locally(&mut self, epochs: u32) -> Result<ModelGradients, FederatedError>;
    fn compress_and_sign_gradients(&self, grad: &ModelGradients) -> [u8; 1024];
    fn update_global_weights(&mut self, global_weights: &[u8]) -> Result<(), FederatedError>;
}

/// Documentation for DistributedVectorStore.
pub trait DistributedVectorStore {
    fn distribute_vector(&mut self, vector: &[f32; 128]) -> Result<(), FederatedError>;
    fn query_cluster_knn(&self, target: &[f32; 128]) -> Option<u32>;
}

/// Documentation for LocalTrainer.
pub trait LocalTrainer {
    fn load_local_dataset(&mut self, data: &[u8]) -> Result<(), FederatedError>;
    fn execute_training_step(&mut self) -> Result<ModelGradients, FederatedError>;
}

/// Documentation for GlobalAggregator.
pub trait GlobalAggregator {
    fn aggregate_gradients(
        &mut self,
        node_gradients: &[ModelGradients],
    ) -> Result<(), FederatedError>;
    fn broadcast_weights(&self) -> [u8; 1024];
}
