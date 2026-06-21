use std::collections::HashMap;

use chrono::Utc;
use parking_lot::RwLock;
use uuid::Uuid;

use shared_types::ConnectorRegistration;
use ztna_core::ZtnaResult;

/// Registered outbound connector metadata.
#[derive(Debug, Clone)]
pub struct Connector {
    pub id: Uuid,
    pub name: String,
    pub endpoint: String,
    pub resource_ids: Vec<Uuid>,
    pub registered_at: chrono::DateTime<Utc>,
}

/// Manages application connector registration and lookup.
pub struct ApplicationConnector {
    connectors: RwLock<HashMap<Uuid, Connector>>,
}

impl ApplicationConnector {
    pub fn new() -> Self {
        Self {
            connectors: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(
        &self,
        name: impl Into<String>,
        endpoint: impl Into<String>,
        resource_ids: Vec<Uuid>,
    ) -> ConnectorRegistration {
        let connector = Connector {
            id: Uuid::new_v4(),
            name: name.into(),
            endpoint: endpoint.into(),
            resource_ids: resource_ids.clone(),
            registered_at: Utc::now(),
        };
        let registration = ConnectorRegistration {
            connector_id: connector.id,
            name: connector.name.clone(),
            endpoint: connector.endpoint.clone(),
            resource_ids,
            registered_at: connector.registered_at,
        };
        self.connectors.write().insert(connector.id, connector);
        registration
    }

    pub fn get(&self, id: Uuid) -> Option<Connector> {
        self.connectors.read().get(&id).cloned()
    }

    pub fn list(&self) -> Vec<Connector> {
        self.connectors.read().values().cloned().collect()
    }

    pub fn unregister(&self, id: Uuid) -> ZtnaResult<()> {
        self.connectors
            .write()
            .remove(&id)
            .map(|_| ())
            .ok_or_else(|| ztna_core::ZtnaError::Connector("connector not found".into()))
    }
}

impl Default for ApplicationConnector {
    fn default() -> Self {
        Self::new()
    }
}
