use publishing::ResourcePublisher;
use shared_types::{Resource, ResourceType, ZtnaSecurityPolicy};

#[test]
fn publish_resource_in_memory() {
    let publisher = ResourcePublisher::in_memory();
    let resource = Resource {
        id: uuid::Uuid::new_v4(),
        name: "wiki".into(),
        resource_type: ResourceType::Https,
        host: "wiki.internal".into(),
        port: 443,
        path_prefix: None,
        tags: vec!["internal".into()],
    };
    let published = publisher.publish("Corporate Wiki", resource);
    assert!(published.published);
    assert_eq!(publisher.list().len(), 1);
}

#[test]
fn attach_access_policy() {
    let publisher = ResourcePublisher::in_memory();
    let resource = Resource {
        id: uuid::Uuid::new_v4(),
        name: "crm".into(),
        resource_type: ResourceType::Http,
        host: "crm.internal".into(),
        port: 80,
        path_prefix: None,
        tags: Vec::new(),
    };
    let published = publisher.publish("CRM", resource.clone());
    let policy = publisher
        .attach_policy(
            published.id,
            ZtnaSecurityPolicy::new("crm-access"),
            vec![],
            vec![],
        )
        .unwrap();
    assert_eq!(policy.resource_id, published.id);
}
