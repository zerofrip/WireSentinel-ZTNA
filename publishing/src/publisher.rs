use std::collections::HashMap;

use chrono::Utc;
use parking_lot::RwLock;
use uuid::Uuid;

use shared_types::{PublishedResource, Resource, ResourceAccessPolicy, ZtnaSecurityPolicy};
use ztna_core::ZtnaResult;

/// In-memory resource publisher with optional SQLite persistence.
pub struct ResourcePublisher {
    resources: RwLock<HashMap<Uuid, PublishedResource>>,
    policies: RwLock<HashMap<Uuid, ResourceAccessPolicy>>,
    #[cfg(feature = "sqlite")]
    pool: Option<sqlx::SqlitePool>,
}

impl ResourcePublisher {
    pub fn in_memory() -> Self {
        Self {
            resources: RwLock::new(HashMap::new()),
            policies: RwLock::new(HashMap::new()),
            #[cfg(feature = "sqlite")]
            pool: None,
        }
    }

    #[cfg(feature = "sqlite")]
    pub async fn with_sqlite(database_url: &str) -> ZtnaResult<Self> {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(2)
            .connect(database_url)
            .await
            .map_err(|e| ztna_core::ZtnaError::Publishing(e.to_string()))?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS published_resources (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                payload TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .map_err(|e| ztna_core::ZtnaError::Publishing(e.to_string()))?;

        Ok(Self {
            resources: RwLock::new(HashMap::new()),
            policies: RwLock::new(HashMap::new()),
            pool: Some(pool),
        })
    }

    pub fn publish(&self, name: impl Into<String>, resource: Resource) -> PublishedResource {
        let published = PublishedResource {
            id: Uuid::new_v4(),
            name: name.into(),
            resource: resource.clone(),
            published: true,
            access_policy_id: None,
            published_at: Utc::now(),
        };
        self.resources
            .write()
            .insert(published.id, published.clone());
        published
    }

    pub fn attach_policy(
        &self,
        resource_id: Uuid,
        policy: ZtnaSecurityPolicy,
        allowed_group_ids: Vec<Uuid>,
        allowed_role_ids: Vec<Uuid>,
    ) -> ZtnaResult<ResourceAccessPolicy> {
        let access = ResourceAccessPolicy {
            id: Uuid::new_v4(),
            resource_id,
            policy,
            allowed_group_ids,
            allowed_role_ids,
        };
        self.policies.write().insert(access.id, access.clone());

        if let Some(entry) = self.resources.write().get_mut(&resource_id) {
            entry.access_policy_id = Some(access.id);
        } else {
            return Err(ztna_core::ZtnaError::Publishing(
                "resource not published".into(),
            ));
        }

        Ok(access)
    }

    pub fn get(&self, id: Uuid) -> Option<PublishedResource> {
        self.resources.read().get(&id).cloned()
    }

    pub fn list(&self) -> Vec<PublishedResource> {
        self.resources.read().values().cloned().collect()
    }

    pub fn policy(&self, id: Uuid) -> Option<ResourceAccessPolicy> {
        self.policies.read().get(&id).cloned()
    }

    #[cfg(feature = "sqlite")]
    pub async fn persist_resource(&self, published: &PublishedResource) -> ZtnaResult<()> {
        let Some(pool) = &self.pool else {
            return Ok(());
        };
        let payload = serde_json::to_string(published)
            .map_err(|e| ztna_core::ZtnaError::Publishing(e.to_string()))?;
        sqlx::query(
            "INSERT OR REPLACE INTO published_resources (id, name, payload) VALUES (?1, ?2, ?3)",
        )
        .bind(published.id.to_string())
        .bind(&published.name)
        .bind(payload)
        .execute(pool)
        .await
        .map_err(|e| ztna_core::ZtnaError::Publishing(e.to_string()))?;
        Ok(())
    }
}

impl Default for ResourcePublisher {
    fn default() -> Self {
        Self::in_memory()
    }
}
