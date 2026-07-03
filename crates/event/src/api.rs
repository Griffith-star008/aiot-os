use crate::internal::EventInternalBroker;
use std::collections::BTreeMap;
use std::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for EventPayload.
pub struct EventPayload {
    /// Documentation for field `event_id`.
    pub event_id: u32,
    /// Documentation for field `timestamp`.
    pub timestamp: u64,
    /// Documentation for field `data`.
    pub data: [u8; 64],
}

/// Documentation for EventBus.
pub trait EventBus {
    fn publish(&mut self, topic: u32, payload: EventPayload);
    fn subscribe(&mut self, topic: u32, callback_id: u32);
    fn route_events(&mut self);
}

/// Documentation for StandardEventBus.
pub struct StandardEventBus {
    pub(crate) broker: EventInternalBroker,
    subscribers: BTreeMap<u32, Vec<u32>>,
    event_queue: Vec<(u32, EventPayload)>,
}

impl StandardEventBus {
    /// Documentation for new.
    pub fn new() -> Self {
        Self {
            broker: EventInternalBroker::new(),
            subscribers: BTreeMap::new(),
            event_queue: Vec::new(),
        }
    }
}

impl Default for StandardEventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus for StandardEventBus {
    fn publish(&mut self, topic: u32, payload: EventPayload) {
        self.event_queue.push((topic, payload));
    }

    fn subscribe(&mut self, topic: u32, callback_id: u32) {
        self.subscribers.entry(topic).or_default().push(callback_id);
    }

    fn route_events(&mut self) {
        let count = self.event_queue.len() as u64;
        self.broker.process(count);
        self.event_queue.clear();
    }
}

/// Documentation for EventStore.
pub trait EventStore {
    fn append_event(&mut self, payload: EventPayload) -> Result<(), &'static str>;
    fn get_event(&self, event_id: u32) -> Option<EventPayload>;
}

/// Documentation for EventReplay.
pub trait EventReplay {
    fn replay_from(&mut self, start_timestamp: u64, end_timestamp: u64);
}

/// Event Bus
use aiot_core::api::AiotError;

#[derive(Clone, Debug)]
pub struct Command {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Query {
    pub query_string: String,
}

#[derive(Clone, Debug)]
pub struct Event {
    pub topic: String,
    pub payload: std::vec::Vec<u8>,
}

