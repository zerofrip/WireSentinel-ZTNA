mod local;
mod mock;
mod oidc;

pub use local::LocalIdentityProvider;
pub use mock::{LdapMockProvider, SamlMockProvider};
pub use oidc::{
    AzureAdProvider, GenericOidcProvider, GoogleWorkspaceProvider, KeycloakProvider, OAuth2Provider,
    OktaProvider,
};
