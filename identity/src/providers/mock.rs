use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use shared_types::{
    GroupIdentity, IdentityAuthResult, IdentityProviderKind, RoleIdentity, UserIdentity,
};
use ztna_core::ZtnaResult;

use crate::provider::IdentityProvider;

/// SAML authentication mock for tests.
pub struct SamlMockProvider;

#[async_trait]
impl IdentityProvider for SamlMockProvider {
    fn kind(&self) -> IdentityProviderKind {
        IdentityProviderKind::SamlMock
    }

    fn display_name(&self) -> &str {
        "SAML Mock"
    }

    async fn authenticate(
        &self,
        username: &str,
        _password: &str,
    ) -> ZtnaResult<IdentityAuthResult> {
        Ok(IdentityAuthResult {
            success: true,
            user: Some(UserIdentity {
                id: Uuid::new_v4(),
                subject: format!("saml:{username}"),
                email: Some(format!("{username}@saml.mock")),
                display_name: username.into(),
                provider: self.kind(),
                authenticated_at: Utc::now(),
            }),
            groups: Vec::new(),
            roles: Vec::new(),
            error: None,
        })
    }

    async fn resolve_user(&self, subject: &str) -> ZtnaResult<Option<UserIdentity>> {
        Ok(Some(UserIdentity {
            id: Uuid::new_v4(),
            subject: subject.into(),
            email: None,
            display_name: subject.into(),
            provider: self.kind(),
            authenticated_at: Utc::now(),
        }))
    }

    async fn list_groups(&self, _user_id: Uuid) -> ZtnaResult<Vec<GroupIdentity>> {
        Ok(Vec::new())
    }

    async fn list_roles(&self, _user_id: Uuid) -> ZtnaResult<Vec<RoleIdentity>> {
        Ok(Vec::new())
    }
}

/// LDAP authentication mock for tests.
pub struct LdapMockProvider;

#[async_trait]
impl IdentityProvider for LdapMockProvider {
    fn kind(&self) -> IdentityProviderKind {
        IdentityProviderKind::LdapMock
    }

    fn display_name(&self) -> &str {
        "LDAP Mock"
    }

    async fn authenticate(&self, username: &str, password: &str) -> ZtnaResult<IdentityAuthResult> {
        let success = password == "ldap-pass";
        Ok(IdentityAuthResult {
            success,
            user: success.then(|| UserIdentity {
                id: Uuid::new_v4(),
                subject: format!("cn={username},dc=example,dc=com"),
                email: Some(format!("{username}@example.com")),
                display_name: username.into(),
                provider: self.kind(),
                authenticated_at: Utc::now(),
            }),
            groups: Vec::new(),
            roles: Vec::new(),
            error: if success {
                None
            } else {
                Some("ldap bind failed".into())
            },
        })
    }

    async fn resolve_user(&self, subject: &str) -> ZtnaResult<Option<UserIdentity>> {
        Ok(Some(UserIdentity {
            id: Uuid::new_v4(),
            subject: subject.into(),
            email: None,
            display_name: subject.into(),
            provider: self.kind(),
            authenticated_at: Utc::now(),
        }))
    }

    async fn list_groups(&self, _user_id: Uuid) -> ZtnaResult<Vec<GroupIdentity>> {
        Ok(vec![GroupIdentity {
            id: Uuid::new_v4(),
            name: "ldap-users".into(),
            external_id: Some("cn=users,dc=example,dc=com".into()),
            provider: self.kind(),
        }])
    }

    async fn list_roles(&self, _user_id: Uuid) -> ZtnaResult<Vec<RoleIdentity>> {
        Ok(Vec::new())
    }
}
