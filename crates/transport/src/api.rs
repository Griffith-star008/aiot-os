//! Public API Interfaces

//! Transport Layer
use aiot_core::api::AiotError;

pub trait Transport {
    fn connect(&mut self) -> Result<(), AiotError>;
    fn disconnect(&mut self) -> Result<(), AiotError>;
}

pub mod protocols {
    pub struct Mqtt;
    pub struct Grpc;
    pub struct Http;
    pub struct Websocket;
    pub struct Quic;
    pub struct Coap;
    pub struct Serial;
    pub struct Can;
    pub struct OpcUa;
}
