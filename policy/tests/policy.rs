use policy::{ConditionalAccessEngine, ZtnaPolicyEngine};
use shared_types::{DevicePosture, DeviceTrustRecord, Subject, SubjectKind, TrustLevel, ZtnaDecision};
use uuid::Uuid;
use ztna_core::ZtnaSecurityPolicy;

#[test]
fn conditional_access_allows_compliant_device() {
    let engine = ConditionalAccessEngine::new();
    let subject = Subject {
        id: Uuid::new_v4(),
        kind: SubjectKind::User,
        display_name: "alice".into(),
        email: Some("alice@example.com".into()),
        group_ids: Vec::new(),
        role_ids: Vec::new(),
        device_id: None,
    };
    let resource_id = Uuid::new_v4();
    let policy = ZtnaSecurityPolicy::new("corp");
    let device = DeviceTrustRecord {
        device_id: Uuid::new_v4(),
        trust_level: TrustLevel::High,
        trust_score: 80,
        posture: DevicePosture {
            compliant: true,
            ..Default::default()
        },
        last_evaluated_at: chrono::Utc::now(),
        certificate_fingerprint: None,
    };

    let result = engine
        .evaluate(&subject, resource_id, &policy, Some(&device))
        .unwrap();
    assert_eq!(result.decision, ZtnaDecision::Allow);
}

#[test]
fn policy_engine_denies_untrusted_device() {
    let engine = ZtnaPolicyEngine::new();
    let subject = Subject {
        id: Uuid::new_v4(),
        kind: SubjectKind::User,
        display_name: "bob".into(),
        email: None,
        group_ids: Vec::new(),
        role_ids: Vec::new(),
        device_id: None,
    };
    let resource_id = Uuid::new_v4();
    let policy = ZtnaSecurityPolicy::new("strict");
    let device = DeviceTrustRecord {
        device_id: Uuid::new_v4(),
        trust_level: TrustLevel::Untrusted,
        trust_score: 10,
        posture: DevicePosture::default(),
        last_evaluated_at: chrono::Utc::now(),
        certificate_fingerprint: None,
    };

    let result = engine
        .evaluate_policies(&subject, resource_id, &[policy], Some(&device))
        .unwrap();
    assert_eq!(result.decision, ZtnaDecision::Deny);
}
