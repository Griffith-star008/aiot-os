use crate::api::{DistributedVectorStore, FederatedError, FederatedNode, ModelGradients};
use std::vec::Vec;

/// Documentation for StandardFederatedNode.
pub struct StandardFederatedNode {
    /// Documentation for field `current_epochs`.
    pub current_epochs: u32,
    /// Documentation for field `global_weights`.
    pub global_weights: [u8; 1024],
}

impl StandardFederatedNode {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self {
            current_epochs: 0,
            global_weights: [0; 1024],
        }
    }
}

impl Default for StandardFederatedNode {
    fn default() -> Self {
        Self::new()
    }
}

impl FederatedNode for StandardFederatedNode {
    fn train_locally(&mut self, epochs: u32) -> Result<ModelGradients, FederatedError> {
        self.current_epochs += epochs;
        if self.current_epochs > 10000 {
            return Err(FederatedError::TrainingDiverged);
        }

        let mut mock_gradients = [0; 1024];
        // Giả lập tính gradient
        for (i, item) in mock_gradients.iter_mut().enumerate() {
            *item = (i % 255) as u8;
        }

        Ok(ModelGradients {
            payload: mock_gradients,
        })
    }

    fn compress_and_sign_gradients(&self, grad: &ModelGradients) -> [u8; 1024] {
        let mut compressed = [0; 1024];
        // Giả lập nén XOR
        for (i, item) in compressed.iter_mut().enumerate() {
            *item = grad.payload[i] ^ 0xAA;
        }
        compressed
    }

    fn update_global_weights(&mut self, global_weights: &[u8]) -> Result<(), FederatedError> {
        if global_weights.len() != 1024 {
            return Err(FederatedError::NetworkTimeout);
        }
        self.global_weights.copy_from_slice(global_weights);
        Ok(())
    }
}

/// Documentation for InMemoryVectorStore.
pub struct InMemoryVectorStore {
    vectors: Vec<[f32; 128]>,
}

impl InMemoryVectorStore {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self {
            vectors: Vec::new(),
        }
    }
}

impl Default for InMemoryVectorStore {
    fn default() -> Self {
        Self::new()
    }
}

impl DistributedVectorStore for InMemoryVectorStore {
    fn distribute_vector(&mut self, vector: &[f32; 128]) -> Result<(), FederatedError> {
        if self.vectors.len() > 1000 {
            return Err(FederatedError::AggregationFailed);
        }
        self.vectors.push(*vector);
        Ok(())
    }

    fn query_cluster_knn(&self, target: &[f32; 128]) -> Option<u32> {
        if self.vectors.is_empty() {
            return None;
        }

        let mut best_index = 0;
        let mut min_dist = f32::MAX;

        for (idx, vec) in self.vectors.iter().enumerate() {
            let mut dist_sq = 0.0;
            for i in 0..128 {
                let diff = vec[i] - target[i];
                dist_sq += diff * diff;
            }
            if dist_sq < min_dist {
                min_dist = dist_sq;
                best_index = idx as u32;
            }
        }

        Some(best_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_federated_node() {
        let mut node = StandardFederatedNode::new();
        let grad = node.train_locally(10).unwrap();
        let comp = node.compress_and_sign_gradients(&grad);
        assert_ne!(grad.payload[0], comp[0]);
    }

    #[test]
    fn test_knn() {
        let mut store = InMemoryVectorStore::new();
        let mut v1 = [0.0; 128];
        v1[0] = 1.0;
        let mut v2 = [0.0; 128];
        v2[0] = 10.0;

        store.distribute_vector(&v1).unwrap();
        store.distribute_vector(&v2).unwrap();

        let mut target = [0.0; 128];
        target[0] = 2.0; // Gần v1 hơn v2

        assert_eq!(store.query_cluster_knn(&target), Some(0));
    }
}
