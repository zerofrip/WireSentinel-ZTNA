use shared_types::{DevicePosture, TrustLevel, TrustScoreSnapshot};
use uuid::Uuid;

/// Computes 0-100 trust scores from device posture signals.
pub struct TrustScoreEngine;

impl TrustScoreEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn compute(&self, posture: &DevicePosture) -> u8 {
        let mut score: i32 = 20;
        if posture.disk_encrypted {
            score += 20;
        }
        if posture.firewall_enabled {
            score += 15;
        }
        if posture.antivirus_running {
            score += 15;
        }
        if posture.compliant {
            score += 20;
        }
        if posture.jailbroken_or_rooted {
            score -= 40;
        }
        if posture.os_version.is_some() {
            score += 10;
        }
        score.clamp(0, 100) as u8
    }

    pub fn level_for_score(&self, score: u8) -> TrustLevel {
        match score {
            0..=24 => TrustLevel::Untrusted,
            25..=49 => TrustLevel::Low,
            50..=74 => TrustLevel::Medium,
            75..=89 => TrustLevel::High,
            _ => TrustLevel::Full,
        }
    }

    pub fn snapshot(&self, device_id: Uuid, posture: &DevicePosture) -> TrustScoreSnapshot {
        let score = self.compute(posture);
        TrustScoreSnapshot {
            device_id,
            score,
            trust_level: self.level_for_score(score),
            captured_at: chrono::Utc::now(),
        }
    }
}

impl Default for TrustScoreEngine {
    fn default() -> Self {
        Self::new()
    }
}
