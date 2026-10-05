# Echora Documentation

This directory is the maintained engineering portal for Echora.

## Authority map

- **Normative/project intent:** [product/PRD.md](product/PRD.md)
- **Approved architectural direction:** [architecture/ARCHITECTURE.md](architecture/ARCHITECTURE.md) and ADRs
- **Technical design:** engineering documents
- **Verification/assurance:** quality documents and test evidence
- **Operational targets:** operations documents
- **Historical changes:** repository changelog and superseded ADRs

If code and documentation disagree, do not silently edit the requirement to match code. Record the divergence and resolve it explicitly.

## Product
- [PRD](product/PRD.md)

## Architecture
- [System architecture](architecture/ARCHITECTURE.md)
- [Visualization architecture](architecture/VISUALIZATION_ARCHITECTURE.md)
- [ADR 0001 — architecture baseline](architecture/adr/0001-architecture-baseline.md)

## Engineering
- [Stack](engineering/STACK.md)
- [Repository structure](engineering/STRUCTURE.md)
- [Sensing pipeline](engineering/SENSING_PIPELINE.md)
- [CSI acquisition and hardware matrix](engineering/CSI_ACQUISITION_AND_HARDWARE_MATRIX.md)
- [RF geometry and Fresnel model](engineering/RF_GEOMETRY_AND_FRESNEL_MODEL.md)
- [Coordinate frames and time synchronization](engineering/COORDINATE_FRAMES_AND_TIME_SYNC.md)
- [Calibration and drift](engineering/CALIBRATION_AND_DRIFT.md)
- [Capability negotiation](engineering/CAPABILITY_NEGOTIATION.md)
- [IEEE 802.11bf compatibility strategy](engineering/IEEE_80211BF_COMPATIBILITY.md)
- [Data and protocol contracts](engineering/DATA_AND_PROTOCOL.md)
- [Experimentation](engineering/EXPERIMENTATION.md)
- [Development workflow](engineering/DEVELOPMENT.md)

## Quality
- [Test strategy](quality/TEST_STRATEGY.md)
- [QA and assurance](quality/QA_ASSURANCE.md)
- [Dataset and benchmark protocol](quality/DATASET_AND_BENCHMARK_PROTOCOL.md)
- [Claim and evidence matrix](quality/CLAIM_EVIDENCE_MATRIX.md)
- [Traceability](quality/TRACEABILITY.md)
- [Release gates](quality/RELEASE_GATES.md)

## Security
- [Security and privacy](security/SECURITY_PRIVACY.md)

## Operations
- [SLO/SLA](operations/SLO_SLA.md)
- [Observability](operations/OBSERVABILITY.md)

## Risk and research
- [Risk register](risk/RISK_REGISTER.md)
- [Related work](research/RELATED_WORK.md)
- [Wi-Fi / RF sensing ecosystem survey](research/ECOSYSTEM_SURVEY.md)

## Machine-readable contracts
- [Sensing result schema](../schemas/sensing-result.schema.json)
- [Run manifest schema](../schemas/run-manifest.schema.json)
- [Sensor topology schema](../schemas/sensor-topology.schema.json)
- [Calibration schema](../schemas/calibration.schema.json)
- [Spatial state schema](../schemas/spatial-state.schema.json)
- [Capability status schema](../schemas/capability-status.schema.json)

## Templates
- [Experiment record](../templates/EXPERIMENT_RECORD.md)
- [Test case](../templates/TEST_CASE.md)
- [ADR](../templates/ADR.md)
- [Model card](../templates/MODEL_CARD.md)
- [Data card](../templates/DATA_CARD.md)
- [Capability evidence](../templates/CAPABILITY_EVIDENCE.md)
- [Hardware validation](../templates/HARDWARE_VALIDATION.md)
- [Calibration report](../templates/CALIBRATION_REPORT.md)

## Project-wide invariant

```text
measurement quality
+ provenance
+ calibration
+ uncertainty
+ evidence scope
> visual impressiveness
```
