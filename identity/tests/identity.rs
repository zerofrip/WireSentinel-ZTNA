use std::sync::Arc;

use identity::{IdentityManager, IdentityProvider, LocalIdentityProvider};
use shared_types::IdentityProviderKind;

#[tokio::test]
async fn local_provider_authenticates() {
    let manager = IdentityManager::new();
    manager.register(Arc::new(LocalIdentityProvider::with_defaults()));
    manager.set_active(IdentityProviderKind::Local);

    let provider = manager.active_provider().expect("active provider");
    let result = provider.authenticate("alice", "password").await.unwrap();
    assert!(result.success);
    assert_eq!(result.user.as_ref().unwrap().subject, "alice");
}

#[tokio::test]
async fn local_provider_rejects_bad_password() {
    let provider = LocalIdentityProvider::with_defaults();
    let result = provider.authenticate("alice", "wrong").await.unwrap();
    assert!(!result.success);
}
