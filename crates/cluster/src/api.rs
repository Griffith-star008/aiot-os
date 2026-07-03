extern crate alloc;

/// Documentation for RaftConsensus.
pub trait RaftConsensus {
    fn request_vote(&mut self) -> bool;
    fn append_entries(&mut self, log_entry: &[u8]) -> Result<(), &'static str>;
    fn create_snapshot(&self) -> [u8; 256];
}

/// Documentation for GossipProtocol.
pub trait GossipProtocol {
    fn broadcast_state(&mut self, state: &[u8; 32]);
    fn receive_gossip(&mut self, neighbor_state: &[u8; 32]);
}

/// Documentation for NodeHealth.
pub struct NodeHealth {
    /// Documentation for field `node_id`.
    pub node_id: u32,
    /// Documentation for field `health_score`.
    pub health_score: u8,
}

/// Documentation for ClusterMembership.
pub trait ClusterMembership {
    fn join_cluster(&mut self, node_id: u32) -> Result<(), &'static str>;
    fn leave_cluster(&mut self, node_id: u32);
    fn check_global_health(&self) -> u8;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for MissionPlacement.
pub enum MissionPlacement {
    LocalNode,
    RemoteNode(u32),
}

/// Documentation for DistributedScheduler.
pub trait DistributedScheduler {
    fn evaluate_placement(&self, mission_size: u32) -> MissionPlacement;
    fn dispatch_execution(
        &mut self,
        target_node: u32,
        mission_payload: &[u8],
    ) -> Result<(), &'static str>;
}

/// Documentation for DistributedLock.
pub trait DistributedLock {
    fn try_acquire(&mut self, resource_id: &str, ttl_ms: u64) -> Result<bool, &'static str>;
    fn release(&mut self, resource_id: &str) -> Result<(), &'static str>;
}

/// Documentation for LeaderElection.
pub trait LeaderElection {
    fn campaign(&mut self, node_id: u32) -> Result<(), &'static str>;
    fn step_down(&mut self, node_id: u32) -> Result<(), &'static str>;
    fn is_leader(&self, node_id: u32) -> bool;
}

/// Documentation for ConsensusProtocol.
pub trait ConsensusProtocol {
    fn propose_value(&mut self, value: &[u8]) -> Result<(), &'static str>;
    fn sync_state(&mut self) -> Result<(), &'static str>;
}

use std::vec::Vec;

/// Documentation for NodeDiscovery.
pub trait NodeDiscovery {
    fn discover_peers(&self) -> Result<Vec<u32>, &'static str>;
    fn register_node(&mut self, ip: &str, port: u16) -> Result<(), &'static str>;
}
