use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;
use shared_types::IdentityProviderKind;

use crate::provider::IdentityProvider;

/// Registry and orchestrator for configured identity providers.
pub struct IdentityManager {
    providers: RwLock<HashMap<IdentityProviderKind, Arc<dyn IdentityProvider>>>,
    active: RwLock<Option<IdentityProviderKind>>,
}

impl IdentityManager {
    pub fn new() -> Self {
        Self {
            providers: RwLock::new(HashMap::new()),
            active: RwLock::new(None),
        }
    }

    pub fn register(&self, provider: Arc<dyn IdentityProvider>) {
        let kind = provider.kind();
        self.providers.write().insert(kind, provider);
    }

    pub fn set_active(&self, kind: IdentityProviderKind) -> bool {
        if self.providers.read().contains_key(&kind) {
            *self.active.write() = Some(kind);
            true
        } else {
            false
        }
    }

    pub fn active_provider(&self) -> Option<Arc<dyn IdentityProvider>> {
        let active = self.active.read().clone()?;
        self.providers.read().get(&active).cloned()
    }

    pub fn provider(&self, kind: IdentityProviderKind) -> Option<Arc<dyn IdentityProvider>> {
        self.providers.read().get(&kind).cloned()
    }

    pub fn registered_kinds(&self) -> Vec<IdentityProviderKind> {
        self.providers.read().keys().cloned().collect()
    }
}

impl Default for IdentityManager {
    fn default() -> Self {
        Self::new()
    }
}
