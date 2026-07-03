use crate::api::{
    AgentMessage, AgentMessageBus, AgentSandbox, AgentSupervisor, SandboxedAgent, SupervisorDomain,
};
use std::collections::BTreeMap;
use std::vec::Vec;

/// Documentation for AgentState.
pub struct AgentState {
    /// Documentation for field `domain`.
    pub domain: SupervisorDomain,
    /// Documentation for field `health`.
    pub health: u8,
}

/// Documentation for StandardAgentSupervisor.
pub struct StandardAgentSupervisor {
    /// Documentation for field `agents`.
    pub agents: BTreeMap<u32, AgentState>,
    next_id: u32,
}

impl StandardAgentSupervisor {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self {
            agents: BTreeMap::new(),
            next_id: 1,
        }
    }
}

impl Default for StandardAgentSupervisor {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentSupervisor for StandardAgentSupervisor {
    fn spawn_agent(&mut self, domain: SupervisorDomain) -> Result<u32, &'static str> {
        if self.agents.len() >= 1000 {
            return Err("Max agents reached");
        }
        let id = self.next_id;
        self.next_id += 1;
        self.agents.insert(
            id,
            AgentState {
                domain,
                health: 100,
            },
        );
        Ok(id)
    }

    fn terminate_agent(&mut self, agent_id: u32) {
        self.agents.remove(&agent_id);
    }

    fn monitor_health(&self, agent_id: u32) -> u8 {
        self.agents.get(&agent_id).map(|s| s.health).unwrap_or(0)
    }
}

/// Documentation for StandardAgentSandbox.
pub struct StandardAgentSandbox {
    /// Documentation for field `sandboxed_agents`.
    pub sandboxed_agents: BTreeMap<u32, SandboxedAgent>,
}

impl StandardAgentSandbox {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self {
            sandboxed_agents: BTreeMap::new(),
        }
    }
}

impl Default for StandardAgentSandbox {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentSandbox for StandardAgentSandbox {
    fn isolate_memory(&mut self, agent_id: u32) -> Result<(), &'static str> {
        self.sandboxed_agents.insert(
            agent_id,
            SandboxedAgent {
                agent_id,
                is_isolated: true,
            },
        );
        Ok(())
    }

    fn restrict_syscalls(&mut self, agent_id: u32) {
        if let Some(agent) = self.sandboxed_agents.get_mut(&agent_id) {
            agent.is_isolated = true; // Simulating syscall restriction
        }
    }

    fn drop_privileges(&mut self, agent_id: u32) {
        if let Some(agent) = self.sandboxed_agents.get_mut(&agent_id) {
            agent.is_isolated = true; // Simulating privilege drop
        }
    }
}

/// Documentation for LocalAgentBus.
pub struct LocalAgentBus {
    /// Documentation for field `messages`.
    pub messages: Vec<AgentMessage>,
}

impl LocalAgentBus {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}

impl Default for LocalAgentBus {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentMessageBus for LocalAgentBus {
    fn route_message(&mut self, msg: AgentMessage) -> Result<(), &'static str> {
        if self.messages.len() > 10000 {
            return Err("Bus overflow");
        }
        self.messages.push(msg);
        Ok(())
    }

    fn broadcast(&mut self, sender_id: u32, payload: &[u8; 128]) {
        self.messages.push(AgentMessage {
            sender_id,
            receiver_id: 0, // 0 for broadcast
            payload: *payload,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_and_terminate() {
        let mut supervisor = StandardAgentSupervisor::new();
        let id = supervisor.spawn_agent(SupervisorDomain::Vision).unwrap();
        assert_eq!(supervisor.monitor_health(id), 100);
        supervisor.terminate_agent(id);
        assert_eq!(supervisor.monitor_health(id), 0);
    }
}
