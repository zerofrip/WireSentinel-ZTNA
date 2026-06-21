use std::collections::HashMap;

use chrono::Utc;
use parking_lot::RwLock;
use uuid::Uuid;

use shared_types::{DevicePosture, DeviceTrustRecord};
use ztna_core::ZtnaResult;

use crate::score::TrustScoreEngine;

/// Maintains device trust records and posture state.
pub struct DeviceTrustEngine {
    records: RwLock<HashMap<Uuid, DeviceTrustRecord>>,
    scorer: TrustScoreEngine,
}

impl DeviceTrustEngine {
    pub fn new() -> Self {
        Self {
            records: RwLock::new(HashMap::new()),
            scorer: TrustScoreEngine::new(),
        }
    }

    pub fn evaluate(
        &self,
        device_id: Uuid,
        posture: DevicePosture,
    ) -> ZtnaResult<DeviceTrustRecord> {
        let score = self.scorer.compute(&posture);
        let trust_level = self.scorer.level_for_score(score);
        let record = DeviceTrustRecord {
            device_id,
            trust_level,
            trust_score: score,
            posture,
            last_evaluated_at: Utc::now(),
            certificate_fingerprint: None,
        };
        self.records.write().insert(device_id, record.clone());
        Ok(record)
    }

    pub fn get(&self, device_id: Uuid) -> Option<DeviceTrustRecord> {
        self.records.read().get(&device_id).cloned()
    }

    pub fn update_posture(
        &self,
        device_id: Uuid,
        posture: DevicePosture,
    ) -> ZtnaResult<DeviceTrustRecord> {
        self.evaluate(device_id, posture)
    }
}

impl Default for DeviceTrustEngine {
    fn default() -> Self {
        Self::new()
    }
}
