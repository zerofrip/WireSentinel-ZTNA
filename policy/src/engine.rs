use uuid::Uuid;

use shared_types::{ConditionalAccessResult, DeviceTrustRecord, Subject, ZtnaSecurityPolicy};
use ztna_core::ZtnaResult;

use crate::conditional::ConditionalAccessEngine;

/// Top-level ZTNA policy engine orchestrating conditional access.
pub struct ZtnaPolicyEngine {
    conditional: ConditionalAccessEngine,
}

impl ZtnaPolicyEngine {
    pub fn new() -> Self {
        Self {
            conditional: ConditionalAccessEngine::new(),
        }
    }

    pub fn evaluate_policies(
        &self,
        subject: &Subject,
        resource_id: Uuid,
        policies: &[ZtnaSecurityPolicy],
        device: Option<&DeviceTrustRecord>,
    ) -> ZtnaResult<ConditionalAccessResult> {
        for policy in policies.iter().filter(|p| p.enabled) {
            let result = self
                .conditional
                .evaluate(subject, resource_id, policy, device)?;
            if result.decision == shared_types::ZtnaDecision::Allow {
                return Ok(result);
            }
        }

        Ok(self
            .conditional
            .evaluate(subject, resource_id, &default_deny_policy(), device)?)
    }
}

impl Default for ZtnaPolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

fn default_deny_policy() -> ZtnaSecurityPolicy {
    ZtnaSecurityPolicy::new("implicit-deny")
}
