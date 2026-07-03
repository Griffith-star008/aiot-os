use crate::api::NetworkMessage;

/// Documentation for NetworkStack.
pub struct NetworkStack {
    /// Documentation for field `active_connections`.
    pub active_connections: u32,
}

impl NetworkStack {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self {
            active_connections: 0,
        }
    }

    /// Documentation for enqueue.
    pub fn enqueue(&mut self, msg: NetworkMessage) -> Result<(), &'static str> {
        // Logic ưu tiên gửi tin nhắn theo QoS
        if msg.payload_size > 1500 {
            return Err("Payload too large for MTU");
        }
        Ok(())
    }
}

impl Default for NetworkStack {
    fn default() -> Self {
        Self::new()
    }
}
