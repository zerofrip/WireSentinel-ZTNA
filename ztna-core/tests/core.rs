use ztna_core::{TrustLevel, ZtnaDecision, ZtnaSecurityPolicy};

#[test]
fn trust_level_score_floor() {
    assert_eq!(TrustLevel::High.score_floor(), 75);
}

#[test]
fn security_policy_defaults_deny() {
    let policy = ZtnaSecurityPolicy::new("default");
    assert!(policy.enabled);
    assert_eq!(policy.min_trust_level, TrustLevel::Medium);
}

#[test]
fn decision_variants_exist() {
    let _ = ZtnaDecision::Allow;
    let _ = ZtnaDecision::Deny;
}
