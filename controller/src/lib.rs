//! Agent heartbeat and policy bundle DTOs for WireSentinel-Controller integration.

mod dto;

pub use dto::ZtnaPolicyBundleDto;
pub use dto::{empty_heartbeat, sample_bundle};
pub use shared_types::ZtnaHeartbeatPayload;
