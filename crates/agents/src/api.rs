#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for SupervisorDomain.
pub enum SupervisorDomain {
    Planner,
    Vision,
    Recovery,
    Security,
}

/// Documentation for AgentSupervisor.
pub trait AgentSupervisor {
    fn spawn_agent(&mut self, domain: SupervisorDomain) -> Result<u32, &'static str>;
    fn terminate_agent(&mut self, agent_id: u32);
    fn monitor_health(&self, agent_id: u32) -> u8; // Điểm 0 - 100
}

/// Documentation for AgentSandbox.
pub trait AgentSandbox {
    fn isolate_memory(&mut self, agent_id: u32) -> Result<(), &'static str>;
    fn restrict_syscalls(&mut self, agent_id: u32);
    fn drop_privileges(&mut self, agent_id: u32);
}

/// Documentation for SandboxedAgent.
pub struct SandboxedAgent {
    /// Documentation for field `agent_id`.
    pub agent_id: u32,
    /// Documentation for field `is_isolated`.
    pub is_isolated: bool,
}

#[derive(Clone, Copy)]
/// Documentation for AgentMessage.
pub struct AgentMessage {
    /// Documentation for field `sender_id`.
    pub sender_id: u32,
    /// Documentation for field `receiver_id`.
    pub receiver_id: u32,
    /// Documentation for field `payload`.
    pub payload: [u8; 128],
}

/// Documentation for AgentMessageBus.
pub trait AgentMessageBus {
    fn route_message(&mut self, msg: AgentMessage) -> Result<(), &'static str>;
    fn broadcast(&mut self, sender_id: u32, payload: &[u8; 128]);
}
