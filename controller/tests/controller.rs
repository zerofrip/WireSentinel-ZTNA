use controller::{empty_heartbeat, sample_bundle, ZtnaPolicyBundleDto};
use uuid::Uuid;

#[test]
fn heartbeat_and_bundle_dto() {
    let agent_id = Uuid::new_v4();
    let heartbeat = empty_heartbeat(agent_id);
    assert_eq!(heartbeat.agent_id, agent_id);

    let dto = ZtnaPolicyBundleDto::from_bundle(sample_bundle(Uuid::new_v4()), Uuid::new_v4());
    assert!(dto.policy_count() >= 1);
}
