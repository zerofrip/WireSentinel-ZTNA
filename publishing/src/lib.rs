//! Resource catalog publishing and access policies.

mod publisher;

pub use publisher::ResourcePublisher;
pub use shared_types::{PublishedResource, Resource, ResourceAccessPolicy, ZtnaSecurityPolicy};
