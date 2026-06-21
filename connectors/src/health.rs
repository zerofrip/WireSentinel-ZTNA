use chrono::Utc;
use uuid::Uuid;

use shared_types::ConnectorHealth;

/// Polls connector health (stub — always healthy unless endpoint empty).
pub struct ConnectorHealthMonitor;

impl ConnectorHealthMonitor {
    pub fn check(connector_id: Uuid, endpoint: &str) -> ConnectorHealth {
        let healthy = !endpoint.is_empty();
        ConnectorHealth {
            connector_id,
            healthy,
            latency_ms: if healthy { Some(12) } else { None },
            last_check_at: Utc::now(),
            message: if healthy {
                Some("reachable".into())
            } else {
                Some("missing endpoint".into())
            },
        }
    }
}
