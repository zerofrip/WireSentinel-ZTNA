use async_trait::async_trait;

use ztna_core::{ZtnaResult, ZtnaSecurityPolicy};

use crate::manifest::ZtnaPluginManifest;

/// Stable hook for Wasm/native ZTNA provider loaders.
#[async_trait]
pub trait ZtnaPlugin: Send + Sync {
    fn manifest(&self) -> &ZtnaPluginManifest;

    /// Validate a policy bundle fragment supplied by the plugin.
    async fn validate_policy(&self, policy: &ZtnaSecurityPolicy) -> ZtnaResult<()>;

    /// Optional initialization hook when the plugin is loaded.
    async fn on_load(&self) -> ZtnaResult<()> {
        Ok(())
    }
}
