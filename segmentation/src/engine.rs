use std::collections::HashMap;

use chrono::Utc;
use parking_lot::RwLock;
use shared_types::{
    IsolationLevel, MicroSegment, SegmentPolicyResult, SegmentType, ServiceEvent, ServiceEventInner,
};
use uuid::Uuid;
use ztna_core::ZtnaResult;

/// Applies segment isolation policies to subject/resource pairs.
pub struct MicroSegmentationEngine {
    segments: RwLock<HashMap<Uuid, MicroSegment>>,
}

impl MicroSegmentationEngine {
    pub fn new() -> Self {
        Self {
            segments: RwLock::new(HashMap::new()),
        }
    }

    pub fn define_segment(
        &self,
        name: impl Into<String>,
        segment_type: SegmentType,
        member_resource_ids: Vec<Uuid>,
        isolation_level: IsolationLevel,
    ) -> MicroSegment {
        let segment = MicroSegment {
            id: Uuid::new_v4(),
            name: name.into(),
            segment_type,
            member_resource_ids,
            isolation_level,
        };
        self.segments.write().insert(segment.id, segment.clone());
        segment
    }

    pub fn apply_policy(
        &self,
        segment_id: Uuid,
        subject_id: Uuid,
        resource_id: Uuid,
        allowed_subjects: &[Uuid],
    ) -> (SegmentPolicyResult, Option<ServiceEvent>) {
        let segments = self.segments.read();
        let Some(segment) = segments.get(&segment_id) else {
            let result = SegmentPolicyResult {
                segment_id,
                subject_id,
                allowed: false,
                reason: "segment not found".into(),
            };
            let event = ServiceEventInner::SegmentPolicyDenied {
                segment_id,
                subject_id,
                reason: result.reason.clone(),
            }
            .with_timestamp(Utc::now());
            return (result, Some(event));
        };

        if !segment.member_resource_ids.contains(&resource_id) {
            let result = SegmentPolicyResult {
                segment_id,
                subject_id,
                allowed: false,
                reason: "resource not in segment".into(),
            };
            let event = ServiceEventInner::SegmentPolicyDenied {
                segment_id,
                subject_id,
                reason: result.reason.clone(),
            }
            .with_timestamp(Utc::now());
            return (result, Some(event));
        }

        let allowed = match segment.isolation_level {
            IsolationLevel::Open => true,
            IsolationLevel::Restricted => allowed_subjects.contains(&subject_id),
            IsolationLevel::Isolated => allowed_subjects.contains(&subject_id),
        };

        let result = SegmentPolicyResult {
            segment_id,
            subject_id,
            allowed,
            reason: if allowed {
                "segment policy applied".into()
            } else {
                "subject not permitted in segment".into()
            },
        };

        let event = if allowed {
            ServiceEventInner::SegmentPolicyApplied {
                result: result.clone(),
            }
            .with_timestamp(Utc::now())
        } else {
            ServiceEventInner::SegmentPolicyDenied {
                segment_id,
                subject_id,
                reason: result.reason.clone(),
            }
            .with_timestamp(Utc::now())
        };

        (result, Some(event))
    }

    pub fn list_segments(&self) -> Vec<MicroSegment> {
        self.segments.read().values().cloned().collect()
    }

    pub fn remove_segment(&self, id: Uuid) -> ZtnaResult<()> {
        self.segments
            .write()
            .remove(&id)
            .map(|_| ())
            .ok_or_else(|| ztna_core::ZtnaError::Segmentation("segment not found".into()))
    }
}

impl Default for MicroSegmentationEngine {
    fn default() -> Self {
        Self::new()
    }
}
