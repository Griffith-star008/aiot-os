use crate::internal::NetworkStack;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for QosLevel.
pub enum QosLevel {
    Background = 0,
    Realtime = 1,
    Critical = 2,
}

/// Documentation for NetworkMessage.
pub struct NetworkMessage {
    /// Documentation for field `payload_size`.
    pub payload_size: u32,
    /// Documentation for field `qos`.
    pub qos: QosLevel,
}

/// Documentation for NetworkManager.
pub struct NetworkManager {
    stack: NetworkStack,
    /// Documentation for field `packets_sent`.
    pub packets_sent: u64,
}

impl NetworkManager {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self {
            stack: NetworkStack::new(),
            packets_sent: 0,
        }
    }

    /// Documentation for send_message.
    pub fn send_message(&mut self, msg: NetworkMessage) -> Result<(), &'static str> {
        self.stack.enqueue(msg)
    }
}

impl Default for NetworkManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Documentation for ApiRouter.
pub trait ApiRouter {
    fn route_v1(&self, path: &str) -> Result<(), &'static str>;
    fn route_v2(&self, path: &str) -> Result<(), &'static str>;
}
