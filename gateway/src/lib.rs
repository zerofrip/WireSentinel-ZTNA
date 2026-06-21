//! ZTNA service gateway — HTTP reverse proxy and TCP relay stubs.

mod gateway;

pub use gateway::{decision_allows, ServiceGateway};
pub use shared_types::{GatewayConnectionRequest, GatewayConnectionResult};
