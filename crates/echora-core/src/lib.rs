#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceMode {
    Live,
    Replay,
    Simulated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceKind {
    Measured,
    Calibrated,
    Reconstructed,
    Inferred,
    Generated,
    Simulated,
    Reference,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability {
    Presence,
    Motion,
    RespirationBpm,
    HeartRateBpm,
    OccupancyCount,
    Position,
    Pose,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbstentionReason {
    InsufficientSnr,
    CalibrationMissing,
    CalibrationStale,
    SourceStale,
    GeometryInsufficient,
    ModelUnavailable,
    ModelIncompatible,
    DataGap,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Provenance {
    pub run_id: String,
    pub capture_id: String,
    pub sensor_ids: Vec<String>,
    pub calibration_id: Option<String>,
    pub model_id: Option<String>,
    pub firmware_version: Option<String>,
    pub config_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SensingResult<T> {
    pub capability: Capability,
    pub value: Option<T>,
    pub confidence: f32,
    pub source_mode: SourceMode,
    pub evidence_kind: EvidenceKind,
    pub quality_flags: Vec<String>,
    pub observed_at_unix_ms: u64,
    pub fresh_until_unix_ms: u64,
    pub abstention: Option<AbstentionReason>,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultValidationError {
    ConfidenceOutOfRange,
    ValueAndAbstentionConflict,
    MissingValueWithoutAbstention,
    FreshnessBeforeObservation,
    SimulatedSourceClaimedMeasured,
}

impl<T> SensingResult<T> {
    pub fn validate(&self) -> Result<(), ResultValidationError> {
        if !self.confidence.is_finite() || !(0.0..=1.0).contains(&self.confidence) {
            return Err(ResultValidationError::ConfidenceOutOfRange);
        }

        if self.value.is_some() && self.abstention.is_some() {
            return Err(ResultValidationError::ValueAndAbstentionConflict);
        }

        if self.value.is_none() && self.abstention.is_none() {
            return Err(ResultValidationError::MissingValueWithoutAbstention);
        }

        if self.fresh_until_unix_ms < self.observed_at_unix_ms {
            return Err(ResultValidationError::FreshnessBeforeObservation);
        }

        if self.source_mode == SourceMode::Simulated
            && matches!(
                self.evidence_kind,
                EvidenceKind::Measured | EvidenceKind::Calibrated | EvidenceKind::Reference
            )
        {
            return Err(ResultValidationError::SimulatedSourceClaimedMeasured);
        }

        Ok(())
    }

    pub fn is_fresh_at(&self, now_unix_ms: u64) -> bool {
        now_unix_ms <= self.fresh_until_unix_ms
    }

    pub fn is_abstained(&self) -> bool {
        self.abstention.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn provenance() -> Provenance {
        Provenance {
            run_id: "run-1".into(),
            capture_id: "capture-1".into(),
            sensor_ids: vec!["node-1".into()],
            calibration_id: Some("cal-1".into()),
            model_id: None,
            firmware_version: Some("0.1.0".into()),
            config_digest: Some("abc".into()),
        }
    }

    #[test]
    fn live_inferred_result_is_valid() {
        let result = SensingResult {
            capability: Capability::Presence,
            value: Some(true),
            confidence: 0.9,
            source_mode: SourceMode::Live,
            evidence_kind: EvidenceKind::Inferred,
            quality_flags: vec![],
            observed_at_unix_ms: 100,
            fresh_until_unix_ms: 500,
            abstention: None,
            provenance: provenance(),
        };

        assert_eq!(result.validate(), Ok(()));
    }

    #[test]
    fn abstention_requires_no_value() {
        let result = SensingResult {
            capability: Capability::RespirationBpm,
            value: Some(14.0_f32),
            confidence: 0.1,
            source_mode: SourceMode::Live,
            evidence_kind: EvidenceKind::Inferred,
            quality_flags: vec![],
            observed_at_unix_ms: 100,
            fresh_until_unix_ms: 500,
            abstention: Some(AbstentionReason::InsufficientSnr),
            provenance: provenance(),
        };

        assert_eq!(
            result.validate(),
            Err(ResultValidationError::ValueAndAbstentionConflict)
        );
    }

    #[test]
    fn simulated_source_cannot_claim_measured_evidence() {
        let result = SensingResult {
            capability: Capability::Presence,
            value: Some(true),
            confidence: 1.0,
            source_mode: SourceMode::Simulated,
            evidence_kind: EvidenceKind::Measured,
            quality_flags: vec![],
            observed_at_unix_ms: 100,
            fresh_until_unix_ms: 500,
            abstention: None,
            provenance: provenance(),
        };

        assert_eq!(
            result.validate(),
            Err(ResultValidationError::SimulatedSourceClaimedMeasured)
        );
    }
}
