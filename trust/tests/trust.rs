use shared_types::DevicePosture;
use trust::{DeviceTrustEngine, TrustScoreEngine};
use uuid::Uuid;
use ztna_core::TrustLevel;

#[test]
fn trust_score_computes_from_posture() {
    let scorer = TrustScoreEngine::new();
    let posture = DevicePosture {
        disk_encrypted: true,
        firewall_enabled: true,
        antivirus_running: true,
        compliant: true,
        os_version: Some("Linux 6.18".into()),
        ..Default::default()
    };
    let score = scorer.compute(&posture);
    assert!(score >= 75);
    assert_eq!(scorer.level_for_score(score), TrustLevel::Full);
}

#[test]
fn device_trust_engine_stores_record() {
    let engine = DeviceTrustEngine::new();
    let device_id = Uuid::new_v4();
    let record = engine
        .evaluate(device_id, DevicePosture::default())
        .unwrap();
    assert_eq!(record.device_id, device_id);
    assert!(engine.get(device_id).is_some());
}
