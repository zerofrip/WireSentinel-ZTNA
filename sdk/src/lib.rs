//! WireSentinel ZTNA SDK — plugin trait and manifest.

mod manifest;
mod plugin;

pub use manifest::ZtnaPluginManifest;
pub use plugin::ZtnaPlugin;
pub use ztna_core::{
    Action, Condition, Resource, Subject, TrustLevel, ZtnaDecision, ZtnaResult, ZtnaSecurityPolicy,
};
