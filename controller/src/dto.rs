use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared_types::{ZtnaHeartbeatPayload, ZtnaPolicyBundle, ZtnaSecurityPolicy};
use uuid::Uuid;

/// Policy bundle DTO pushed from WireSentinel-Controller to agents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZtnaPolicyBundleDto {
    pub bundle: ZtnaPolicyBundle,
    pub controller_id: Uuid,
    pub signature: Option<String>,
    pub received_at: DateTime<Utc>,
}

impl ZtnaPolicyBundleDto {
    pub fn from_bundle(bundle: ZtnaPolicyBundle, controller_id: Uuid) -> Self {
        Self {
            bundle,
            controller_id,
            signature: None,
            received_at: Utc::now(),
        }
    }

    pub fn policy_count(&self) -> usize {
        self.bundle.policies.len()
    }
}

/// Build an empty heartbeat payload for an agent.
pub fn empty_heartbeat(agent_id: Uuid) -> ZtnaHeartbeatPayload {
    ZtnaHeartbeatPayload::empty(agent_id)
}
pub fn sample_bundle(tenant_id: Uuid) -> ZtnaPolicyBundle {
    ZtnaPolicyBundle {
        bundle_id: Uuid::new_v4(),
        tenant_id,
        policies: vec![ZtnaSecurityPolicy::new("default")],
        published_resources: Vec::new(),
        segments: Vec::new(),
        issued_at: Utc::now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_heartbeat_defaults() {
        let id = Uuid::new_v4();
        let payload = empty_heartbeat(id);
        assert_eq!(payload.agent_id, id);
        assert!(!payload.identity_connected);
    }

    #[test]
    fn policy_bundle_dto_counts_policies() {
        let dto = ZtnaPolicyBundleDto::from_bundle(sample_bundle(Uuid::new_v4()), Uuid::new_v4());
        assert_eq!(dto.policy_count(), 1);
    }
}
