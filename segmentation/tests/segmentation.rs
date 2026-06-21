use segmentation::{MicroSegmentationEngine, SegmentType};
use shared_types::IsolationLevel;
use uuid::Uuid;

#[test]
fn segment_policy_applied_for_open_segment() {
    let engine = MicroSegmentationEngine::new();
    let resource_id = Uuid::new_v4();
    let segment = engine.define_segment(
        "apps",
        SegmentType::Application,
        vec![resource_id],
        IsolationLevel::Open,
    );
    let subject_id = Uuid::new_v4();
    let (result, event) = engine.apply_policy(segment.id, subject_id, resource_id, &[]);
    assert!(result.allowed);
    assert!(event.is_some());
}

#[test]
fn segment_policy_denied_for_isolated_segment() {
    let engine = MicroSegmentationEngine::new();
    let resource_id = Uuid::new_v4();
    let segment = engine.define_segment(
        "finance",
        SegmentType::Data,
        vec![resource_id],
        IsolationLevel::Isolated,
    );
    let subject_id = Uuid::new_v4();
    let (result, _) = engine.apply_policy(segment.id, subject_id, resource_id, &[]);
    assert!(!result.allowed);
}
