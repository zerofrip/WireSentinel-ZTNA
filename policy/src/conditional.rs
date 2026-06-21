use chrono::Utc;
use uuid::Uuid;

use shared_types::{
    Condition, ConditionalAccessResult, DeviceTrustRecord, Subject, ZtnaDecision, ZtnaSecurityPolicy,
};
use ztna_core::ZtnaResult;

/// Evaluates conditional access rules against subject and device context.
pub struct ConditionalAccessEngine;

impl ConditionalAccessEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(
        &self,
        subject: &Subject,
        resource_id: Uuid,
        policy: &ZtnaSecurityPolicy,
        device: Option<&DeviceTrustRecord>,
    ) -> ZtnaResult<ConditionalAccessResult> {
        if !policy.enabled {
            return Ok(deny(subject.id, resource_id, policy.id, "policy disabled"));
        }

        if let Some(device) = device {
            if device.trust_level < policy.min_trust_level {
                return Ok(deny(
                    subject.id,
                    resource_id,
                    policy.id,
                    "device trust level below minimum",
                ));
            }
            if device.trust_score < policy.min_trust_score {
                return Ok(deny(
                    subject.id,
                    resource_id,
                    policy.id,
                    "device trust score below minimum",
                ));
            }
        }

        for condition in &policy.conditions {
            if !self.check_condition(condition, subject, device) {
                return Ok(deny(
                    subject.id,
                    resource_id,
                    policy.id,
                    "condition not satisfied",
                ));
            }
        }

        Ok(ConditionalAccessResult {
            decision: ZtnaDecision::Allow,
            subject_id: subject.id,
            resource_id,
            matched_policy_id: Some(policy.id),
            reason: "all conditions satisfied".into(),
            evaluated_at: Utc::now(),
        })
    }

    fn check_condition(
        &self,
        condition: &Condition,
        subject: &Subject,
        device: Option<&DeviceTrustRecord>,
    ) -> bool {
        match condition {
            Condition::TrustLevelAtLeast { level } => {
                device.map(|d| d.trust_level >= *level).unwrap_or(false)
            }
            Condition::TrustScoreAtLeast { score } => {
                device.map(|d| d.trust_score >= *score).unwrap_or(false)
            }
            Condition::GroupMembership { group_id } => subject.group_ids.contains(group_id),
            Condition::RoleAssignment { role_id } => subject.role_ids.contains(role_id),
            Condition::GeoAllowed { countries: _ } => true,
            Condition::TimeWindow { start_hour, end_hour } => {
                let hour = Utc::now().format("%H").to_string().parse::<u8>().unwrap_or(0);
                hour >= *start_hour && hour <= *end_hour
            }
            Condition::DevicePosture { requirement } => device
                .map(|d| d.posture.compliant && requirement == "compliant")
                .unwrap_or(false),
            Condition::Custom { .. } => true,
        }
    }
}

impl Default for ConditionalAccessEngine {
    fn default() -> Self {
        Self::new()
    }
}

fn deny(
    subject_id: Uuid,
    resource_id: Uuid,
    policy_id: Uuid,
    reason: &str,
) -> ConditionalAccessResult {
    ConditionalAccessResult {
        decision: ZtnaDecision::Deny,
        subject_id,
        resource_id,
        matched_policy_id: Some(policy_id),
        reason: reason.into(),
        evaluated_at: Utc::now(),
    }
}
