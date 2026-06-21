//! Device trust and posture scoring for WireSentinel ZTNA.

mod engine;
mod score;

pub use engine::DeviceTrustEngine;
pub use score::TrustScoreEngine;
pub use shared_types::{DevicePosture, DeviceTrustRecord, TrustLevel, TrustScoreSnapshot};
