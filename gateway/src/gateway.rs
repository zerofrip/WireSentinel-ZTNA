use std::net::SocketAddr;
use std::sync::atomic::{AtomicU16, Ordering};

use axum::{routing::get, Router};
use chrono::Utc;
use shared_types::{
    GatewayConnectionRequest, GatewayConnectionResult, Resource, ResourceType, ServiceEvent,
    ServiceEventInner, ZtnaDecision,
};
use tokio::net::TcpListener;
use uuid::Uuid;
use ztna_core::ZtnaResult;

static NEXT_PORT: AtomicU16 = AtomicU16::new(18080);

/// HTTP/HTTPS reverse proxy and TCP relay gateway stub.
pub struct ServiceGateway {
    id: Uuid,
    listen_port: Option<u16>,
}

impl ServiceGateway {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            listen_port: None,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn listen_port(&self) -> Option<u16> {
        self.listen_port
    }

    /// Attempt to establish a proxied connection; emits established/denied events.
    pub fn connect(
        &mut self,
        request: &GatewayConnectionRequest,
        allowed: bool,
        resource: &Resource,
    ) -> (GatewayConnectionResult, Option<ServiceEvent>) {
        let _ = resource;
        if !allowed {
            let event = ServiceEventInner::GatewayConnectionDenied {
                gateway_id: request.gateway_id,
                subject_id: request.subject_id,
                resource_id: request.resource_id,
                reason: "access denied by policy".into(),
            }
            .with_timestamp(Utc::now());
            return (
                GatewayConnectionResult {
                    allowed: false,
                    gateway_id: request.gateway_id,
                    resource_id: request.resource_id,
                    subject_id: request.subject_id,
                    listen_port: None,
                    reason: Some("access denied by policy".into()),
                },
                Some(event),
            );
        }

        let port = NEXT_PORT.fetch_add(1, Ordering::Relaxed);
        self.listen_port = Some(port);

        let event = ServiceEventInner::GatewayConnectionEstablished {
            result: GatewayConnectionResult {
                allowed: true,
                gateway_id: request.gateway_id,
                resource_id: request.resource_id,
                subject_id: request.subject_id,
                listen_port: Some(port),
                reason: None,
            },
        }
        .with_timestamp(Utc::now());

        (
            GatewayConnectionResult {
                allowed: true,
                gateway_id: request.gateway_id,
                resource_id: request.resource_id,
                subject_id: request.subject_id,
                listen_port: Some(port),
                reason: None,
            },
            Some(event),
        )
    }

    /// Start axum reverse-proxy stub bound to loopback.
    pub async fn start_http_stub(&self) -> ZtnaResult<SocketAddr> {
        let port = self.listen_port.unwrap_or(18080);
        let app = Router::new().route(
            "/",
            get(|| async { "WireSentinel ZTNA gateway stub" }),
        );
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let listener = TcpListener::bind(addr).await.map_err(|e| {
            ztna_core::ZtnaError::Gateway(e.to_string())
        })?;
        let bound = listener.local_addr().map_err(|e| {
            ztna_core::ZtnaError::Gateway(e.to_string())
        })?;
        tokio::spawn(async move {
            if axum::serve(listener, app).await.is_err() {
                tracing::warn!("gateway stub stopped");
            }
        });
        Ok(bound)
    }

    /// TCP relay stub — records intent without forwarding bytes.
    pub fn tcp_relay_stub(&self, resource: &Resource) -> ZtnaResult<u16> {
        if !matches!(resource.resource_type, ResourceType::Tcp | ResourceType::Ssh | ResourceType::Database) {
            return Err(ztna_core::ZtnaError::Gateway(
                "resource type does not support TCP relay".into(),
            ));
        }
        Ok(self.listen_port.unwrap_or(19000))
    }
}

impl Default for ServiceGateway {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper for policy engines to gate gateway connections.
pub fn decision_allows(decision: ZtnaDecision) -> bool {
    matches!(decision, ZtnaDecision::Allow)
}
