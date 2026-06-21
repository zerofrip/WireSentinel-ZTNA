//! Identity management and provider integrations for WireSentinel ZTNA.

mod manager;
mod provider;
mod providers;

pub use manager::IdentityManager;
pub use provider::IdentityProvider;
pub use providers::*;
pub use shared_types::{
    GroupIdentity, IdentityAuthResult, IdentityProviderKind, RoleIdentity, UserIdentity,
};
