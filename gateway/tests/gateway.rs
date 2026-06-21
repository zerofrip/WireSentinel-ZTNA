use gateway::{decision_allows, ServiceGateway};
use shared_types::{GatewayConnectionRequest, Resource, ResourceType, ZtnaDecision};
use uuid::Uuid;

#[test]
fn gateway_denied_emits_event() {
    let mut gw = ServiceGateway::new();
    let request = GatewayConnectionRequest {
        gateway_id: gw.id(),
        subject_id: Uuid::new_v4(),
        resource_id: Uuid::new_v4(),
        client_ip: Some("127.0.0.1".into()),
    };
    let resource = Resource {
        id: request.resource_id,
        name: "app".into(),
        resource_type: ResourceType::Https,
        host: "internal.local".into(),
        port: 443,
        path_prefix: None,
        tags: Vec::new(),
    };
    let (result, event) = gw.connect(&request, false, &resource);
    assert!(!result.allowed);
    assert!(event.is_some());
}

#[test]
fn gateway_allowed_assigns_port() {
    let mut gw = ServiceGateway::new();
    let request = GatewayConnectionRequest {
        gateway_id: gw.id(),
        subject_id: Uuid::new_v4(),
        resource_id: Uuid::new_v4(),
        client_ip: None,
    };
    let resource = Resource {
        id: request.resource_id,
        name: "api".into(),
        resource_type: ResourceType::Http,
        host: "api.internal".into(),
        port: 8080,
        path_prefix: Some("/v1".into()),
        tags: Vec::new(),
    };
    let (result, _) = gw.connect(&request, decision_allows(ZtnaDecision::Allow), &resource);
    assert!(result.allowed);
    assert!(result.listen_port.is_some());
}
