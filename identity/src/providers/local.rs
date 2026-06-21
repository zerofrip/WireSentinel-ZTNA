use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use shared_types::{
    GroupIdentity, IdentityAuthResult, IdentityProviderKind, RoleIdentity, UserIdentity,
};
use ztna_core::ZtnaResult;

use crate::provider::IdentityProvider;

/// In-memory local identity provider for development and tests.
pub struct LocalIdentityProvider {
    users: Vec<(String, String, String)>,
}

impl LocalIdentityProvider {
    pub fn with_defaults() -> Self {
        Self {
            users: vec![
                (
                    "alice".into(),
                    "password".into(),
                    "alice@example.com".into(),
                ),
                ("bob".into(), "password".into(), "bob@example.com".into()),
            ],
        }
    }
}

#[async_trait]
impl IdentityProvider for LocalIdentityProvider {
    fn kind(&self) -> IdentityProviderKind {
        IdentityProviderKind::Local
    }

    fn display_name(&self) -> &str {
        "Local"
    }

    async fn authenticate(&self, username: &str, password: &str) -> ZtnaResult<IdentityAuthResult> {
        if let Some((_, pass, email)) = self.users.iter().find(|(u, _, _)| u == username) {
            if pass == password {
                return Ok(IdentityAuthResult {
                    success: true,
                    user: Some(UserIdentity {
                        id: Uuid::new_v4(),
                        subject: username.into(),
                        email: Some(email.clone()),
                        display_name: username.into(),
                        provider: self.kind(),
                        authenticated_at: Utc::now(),
                    }),
                    groups: vec![GroupIdentity {
                        id: Uuid::new_v4(),
                        name: "local-users".into(),
                        external_id: None,
                        provider: self.kind(),
                    }],
                    roles: vec![RoleIdentity {
                        id: Uuid::new_v4(),
                        name: "user".into(),
                        permissions: vec!["access:basic".into()],
                        provider: self.kind(),
                    }],
                    error: None,
                });
            }
        }
        Ok(IdentityAuthResult {
            success: false,
            user: None,
            groups: Vec::new(),
            roles: Vec::new(),
            error: Some("invalid credentials".into()),
        })
    }

    async fn resolve_user(&self, subject: &str) -> ZtnaResult<Option<UserIdentity>> {
        Ok(self
            .users
            .iter()
            .find(|(u, _, _)| u == subject)
            .map(|(u, _, email)| UserIdentity {
                id: Uuid::new_v4(),
                subject: u.clone(),
                email: Some(email.clone()),
                display_name: u.clone(),
                provider: self.kind(),
                authenticated_at: Utc::now(),
            }))
    }

    async fn list_groups(&self, _user_id: Uuid) -> ZtnaResult<Vec<GroupIdentity>> {
        Ok(vec![GroupIdentity {
            id: Uuid::new_v4(),
            name: "local-users".into(),
            external_id: None,
            provider: self.kind(),
        }])
    }

    async fn list_roles(&self, _user_id: Uuid) -> ZtnaResult<Vec<RoleIdentity>> {
        Ok(vec![RoleIdentity {
            id: Uuid::new_v4(),
            name: "user".into(),
            permissions: vec!["access:basic".into()],
            provider: self.kind(),
        }])
    }
}
