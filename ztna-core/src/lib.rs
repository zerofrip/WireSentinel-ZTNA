//! Core ZTNA abstractions for WireSentinel Phase 15.

mod error;

pub use error::{ZtnaError, ZtnaResult};
pub use shared_types::{
    Action, Condition, Resource, ResourceType, Subject, SubjectKind, TrustLevel, ZtnaDecision,
    ZtnaSecurityPolicy,
};
