use async_trait::async_trait;

use shared_types::{GroupIdentity, IdentityAuthResult, IdentityProviderKind, RoleIdentity, UserIdentity};
use ztna_core::ZtnaResult;

/// Pluggable identity provider interface.
#[async_trait]
pub trait IdentityProvider: Send + Sync {
    fn kind(&self) -> IdentityProviderKind;

    fn display_name(&self) -> &str;

    async fn authenticate(&self, username: &str, password: &str) -> ZtnaResult<IdentityAuthResult>;

    async fn resolve_user(&self, subject: &str) -> ZtnaResult<Option<UserIdentity>>;

    async fn list_groups(&self, user_id: uuid::Uuid) -> ZtnaResult<Vec<GroupIdentity>>;

    async fn list_roles(&self, user_id: uuid::Uuid) -> ZtnaResult<Vec<RoleIdentity>>;
}
