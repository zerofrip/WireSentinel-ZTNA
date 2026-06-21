use std::collections::VecDeque;

use chrono::Utc;
use parking_lot::Mutex;
use uuid::Uuid;

use shared_types::{ZtnaAccessDecisionRecord, ZtnaAnalyticsSnapshot, ZtnaDecision};

const DEFAULT_CAPACITY: usize = 10_000;

/// Records and aggregates ZTNA access decisions.
pub struct ZtnaAnalytics {
    records: Mutex<VecDeque<ZtnaAccessDecisionRecord>>,
    capacity: usize,
}

impl ZtnaAnalytics {
    pub fn new() -> Self {
        Self {
            records: Mutex::new(VecDeque::new()),
            capacity: DEFAULT_CAPACITY,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            records: Mutex::new(VecDeque::new()),
            capacity,
        }
    }

    pub fn record_decision(
        &self,
        subject_id: Uuid,
        resource_id: Uuid,
        decision: ZtnaDecision,
        trust_score: u8,
        reason: impl Into<String>,
    ) -> ZtnaAccessDecisionRecord {
        let record = ZtnaAccessDecisionRecord {
            id: Uuid::new_v4(),
            subject_id,
            resource_id,
            decision,
            trust_score,
            reason: reason.into(),
            recorded_at: Utc::now(),
        };
        let mut records = self.records.lock();
        if records.len() >= self.capacity {
            records.pop_front();
        }
        records.push_back(record.clone());
        record
    }

    pub fn snapshot(&self) -> ZtnaAnalyticsSnapshot {
        let records = self.records.lock();
        let total = records.len() as u64;
        let allow_count = records
            .iter()
            .filter(|r| r.decision == ZtnaDecision::Allow)
            .count() as u64;
        let deny_count = records
            .iter()
            .filter(|r| r.decision == ZtnaDecision::Deny)
            .count() as u64;
        let challenge_count = records
            .iter()
            .filter(|r| matches!(r.decision, ZtnaDecision::Challenge | ZtnaDecision::StepUp))
            .count() as u64;
        let avg_trust_score = if total == 0 {
            0.0
        } else {
            records.iter().map(|r| r.trust_score as f64).sum::<f64>() / total as f64
        };

        ZtnaAnalyticsSnapshot {
            total_decisions: total,
            allow_count,
            deny_count,
            challenge_count,
            avg_trust_score,
            captured_at: Utc::now(),
        }
    }

    pub fn recent(&self, limit: usize) -> Vec<ZtnaAccessDecisionRecord> {
        self.records
            .lock()
            .iter()
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }
}

impl Default for ZtnaAnalytics {
    fn default() -> Self {
        Self::new()
    }
}
