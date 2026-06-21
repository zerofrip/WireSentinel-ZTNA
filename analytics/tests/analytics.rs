use analytics::ZtnaAnalytics;
use shared_types::ZtnaDecision;
use uuid::Uuid;

#[test]
fn analytics_records_decisions() {
    let analytics = ZtnaAnalytics::new();
    analytics.record_decision(
        Uuid::new_v4(),
        Uuid::new_v4(),
        ZtnaDecision::Allow,
        80,
        "ok",
    );
    analytics.record_decision(
        Uuid::new_v4(),
        Uuid::new_v4(),
        ZtnaDecision::Deny,
        20,
        "low trust",
    );
    let snapshot = analytics.snapshot();
    assert_eq!(snapshot.total_decisions, 2);
    assert_eq!(snapshot.allow_count, 1);
    assert_eq!(snapshot.deny_count, 1);
}
