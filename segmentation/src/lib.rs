//! Micro-segmentation engine for ZTNA workloads.

mod engine;

pub use engine::MicroSegmentationEngine;
pub use shared_types::{
    IsolationLevel, MicroSegment, SegmentPolicyResult, SegmentType, ServiceEvent, ServiceEventInner,
};
