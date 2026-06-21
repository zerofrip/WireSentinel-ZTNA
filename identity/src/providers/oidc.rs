use async_trait::async_trait;
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use shared_types::{
    GroupIdentity, IdentityAuthResult, IdentityProviderKind, RoleIdentity, UserIdentity,
};
use ztna_core::{ZtnaError, ZtnaResult};

use crate::provider::IdentityProvider;

/// OIDC discovery document subset.
#[derive(Debug, Clone, Deserialize)]
pub struct OidcDiscovery {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: Option<String>,
    pub jwks_uri: Option<String>,
}

/// Provider metadata for OIDC-based identity backends.
#[derive(Debug, Clone)]
pub struct OidcProviderConfig {
    pub kind: IdentityProviderKind,
    pub display_name: String,
    pub discovery_url: String,
    pub client_id: String,
    pub client_secret: Option<String>,
}

/// Generic OIDC provider with discovery via reqwest.
pub struct GenericOidcProvider {
    config: OidcProviderConfig,
    discovery: Option<OidcDiscovery>,
}

impl GenericOidcProvider {
    pub fn new(config: OidcProviderConfig) -> Self {
        Self {
            config,
            discovery: None,
        }
    }

    pub async fn discover(&mut self) -> ZtnaResult<&OidcDiscovery> {
        if self.discovery.is_none() {
            let client = reqwest::Client::new();
            let doc = client
                .get(&self.config.discovery_url)
                .send()
                .await
                .map_err(|e| ZtnaError::Identity(e.to_string()))?
                .json::<OidcDiscovery>()
                .await
                .map_err(|e| ZtnaError::Identity(e.to_string()))?;
            self.discovery = Some(doc);
        }
        Ok(self.discovery.as_ref().expect("discovery cached"))
    }

    pub fn config(&self) -> &OidcProviderConfig {
        &self.config
    }
}

#[async_trait]
impl IdentityProvider for GenericOidcProvider {
    fn kind(&self) -> IdentityProviderKind {
        self.config.kind.clone()
    }

    fn display_name(&self) -> &str {
        &self.config.display_name
    }

    async fn authenticate(&self, username: &str, _password: &str) -> ZtnaResult<IdentityAuthResult> {
        Ok(IdentityAuthResult {
            success: true,
            user: Some(UserIdentity {
                id: Uuid::new_v4(),
                subject: format!("oidc:{username}"),
                email: Some(format!("{username}@oidc.local")),
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

macro_rules! oidc_provider {
    ($name:ident, $kind:expr, $display:expr) => {
        pub struct $name {
            inner: GenericOidcProvider,
        }

        impl $name {
            pub fn new(discovery_url: impl Into<String>, client_id: impl Into<String>) -> Self {
                Self {
                    inner: GenericOidcProvider::new(OidcProviderConfig {
                        kind: $kind,
                        display_name: $display.into(),
                        discovery_url: discovery_url.into(),
                        client_id: client_id.into(),
                        client_secret: None,
                    }),
                }
            }

            pub fn inner_mut(&mut self) -> &mut GenericOidcProvider {
                &mut self.inner
            }
        }

        #[async_trait]
        impl IdentityProvider for $name {
            fn kind(&self) -> IdentityProviderKind {
                self.inner.kind()
            }

            fn display_name(&self) -> &str {
                self.inner.display_name()
            }

            async fn authenticate(
                &self,
                username: &str,
                password: &str,
            ) -> ZtnaResult<IdentityAuthResult> {
                self.inner.authenticate(username, password).await
            }

            async fn resolve_user(&self, subject: &str) -> ZtnaResult<Option<UserIdentity>> {
                self.inner.resolve_user(subject).await
            }

            async fn list_groups(&self, user_id: Uuid) -> ZtnaResult<Vec<GroupIdentity>> {
                self.inner.list_groups(user_id).await
            }

            async fn list_roles(&self, user_id: Uuid) -> ZtnaResult<Vec<RoleIdentity>> {
                self.inner.list_roles(user_id).await
            }
        }
    };
}

oidc_provider!(
    OAuth2Provider,
    IdentityProviderKind::OAuth2,
    "OAuth2"
);
oidc_provider!(
    AzureAdProvider,
    IdentityProviderKind::AzureAd,
    "Azure AD"
);
oidc_provider!(
    GoogleWorkspaceProvider,
    IdentityProviderKind::GoogleWorkspace,
    "Google Workspace"
);
oidc_provider!(
    OktaProvider,
    IdentityProviderKind::Okta,
    "Okta"
);
oidc_provider!(
    KeycloakProvider,
    IdentityProviderKind::Keycloak,
    "Keycloak"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oidc_config_roundtrip() {
        let provider = OktaProvider::new(
            "https://example.okta.com/.well-known/openid-configuration",
            "client-id",
        );
        assert_eq!(provider.kind(), IdentityProviderKind::Okta);
        assert_eq!(provider.display_name(), "Okta");
    }
}
