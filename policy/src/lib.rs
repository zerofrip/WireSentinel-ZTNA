//! Conditional access and ZTNA policy evaluation.

mod conditional;
mod engine;

pub use conditional::ConditionalAccessEngine;
pub use engine::ZtnaPolicyEngine;
pub use shared_types::{ConditionalAccessResult, ZtnaDecision, ZtnaSecurityPolicy};
