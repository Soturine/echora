# Echora

> **Device-free RF human sensing, built around evidence rather than demos.**

Echora is an open-source research and engineering project for detecting and estimating human presence, motion, occupancy, localization, respiration and, later, body pose from radio signals without requiring a camera or wearable during normal operation.

The first sensing modality is **Wi-Fi Channel State Information (CSI)** captured by low-cost ESP32-S3/ESP32-C6 nodes. The architecture is intentionally modality-neutral so future adapters can evaluate mmWave radar, UWB, BLE Channel Sounding or other RF sources without rewriting the system.

## Project intent

Echora is designed to answer a harder question than “can we draw a person on a dashboard?”:

> **What can the current sensor topology actually support, with what uncertainty, under which environment, calibration, hardware, model and evidence?**

A complete-looking visualization is never treated as proof. Echora keeps direct measurements, calibrated measurements, reconstructions, inferences, generated data and simulation explicitly separate.

## Planned capability path

| Phase | Capability | Primary evidence target |
|---|---|---|
| P0 | Real CSI capture and replay | raw frames, timestamps, loss, hardware provenance |
| P1 | Presence, motion, respiration | empty-room controls + independent reference |
| P2 | Multi-node occupancy and coarse localization | known positions + held-out room geometry |
| P3 | Synchronized reference capture | Kinect/RGB-D/camera used as training/reference instrumentation |
| P4 | Camera-free pose inference | held-out people/rooms/sessions, calibrated uncertainty and abstention |
| P5 | Cardiac micro-motion research | independent ECG/PPG/watch reference; experimental until validated |
| P6 | SpatialTwin integration | RF-derived human state placed in a metric 3D environment with provenance |

**Current repository maturity:** specification / architecture foundation with a minimal Rust evidence/protocol scaffold. A capability is not considered implemented merely because it appears in the roadmap or documentation.

## Evidence vocabulary

Echora adopts an evidence-governed sensing model:

```text
MEASURED
CALIBRATED
RECONSTRUCTED
INFERRED
GENERATED
SIMULATED
REFERENCE
```

Live, replay and synthetic sources are also distinct. A simulator must never silently masquerade as a live sensor.

Every meaningful sensing output is expected to be traceable to fields such as:

```text
value
confidence
source_mode
evidence_kind
sensor_ids
capture_id
calibration_id
model_version
firmware_version
environment_id
timestamp
quality_flags
reference_status
```

When evidence is insufficient, **abstention is a valid result**. Returning `null + reason=INSUFFICIENT_SNR` is preferable to emitting a plausible but unsupported number.

## Initial technical direction

```text
ESP32-S3 / ESP32-C6
        │
        │ raw CSI + metadata
        ▼
RF Ingest / Gateway
Rust
        │
        ├── protocol validation
        ├── time alignment
        ├── packet-loss accounting
        ├── calibration identity
        └── provenance
        ▼
Signal Processing
Rust
        │
        ├── amplitude / phase
        ├── sanitization / outlier rejection
        ├── subcarrier selection
        ├── spectral / temporal features
        └── uncertainty / quality
        ▼
Inference & Fusion
Rust runtime + Python training
        │
        ├── presence
        ├── motion
        ├── respiration
        ├── occupancy / localization
        └── pose candidates
        ▼
Spatial State
        │
        ├── confidence
        ├── abstention
        ├── temporal tracking
        └── room/world coordinates
        ▼
API / WebSocket / MQTT
        ▼
Web dashboard / Three.js / SpatialTwin
```

Baseline stack decisions and alternatives are documented under [docs/engineering/STACK.md](docs/engineering/STACK.md).

## Engineering model

This repository follows the **Derivanta / Engineering Constitution** principles used across Soturine engineering work:

- problem and evidence before technology;
- modular architecture with explicit boundaries and contracts;
- raw-source provenance and reproducible experiments;
- `implemented ≠ tested ≠ integrated ≠ validated`;
- `simulation ≠ hardware validation`;
- `measured ≠ reconstructed ≠ inferred ≠ generated`;
- deterministic handling of critical invariants;
- small reviewable changes and logical commits;
- verification and validation as different activities;
- negative, edge, failure and degraded-state testing;
- security/privacy/supply-chain controls throughout the lifecycle;
- exact-revision evidence for releases;
- observability, run lineage and calibration identity;
- documentation authority, ADRs and explicit gaps;
- no unsupported claims promoted from one favorable run.

See [AGENTS.md](AGENTS.md) and [docs/README.md](docs/README.md).

## Repository direction

```text
docs/           product, architecture, engineering, QA, security and operations
firmware/       ESP-IDF capture nodes
crates/         Rust core, protocol, DSP, fusion, API and tooling
ml/             training, evaluation and model export
web/            operator/research UI and spatial visualization
schemas/        machine-readable contracts and evidence envelopes
hardware/       BOM, topology, calibration fixtures and setup
data/           dataset/capture format documentation (not raw personal captures)
tests/          cross-layer replay, HIL and acceptance harnesses
templates/      ADR, test and experiment records
```

Directories are introduced when their first authoritative artifact or implementation exists; empty architecture is not evidence of implementation.

## Current code foundation

The repository currently includes:
- `echora-core`: source/evidence/provenance/abstention domain types;
- `echora-protocol`: bounded v1 binary sensor frame codec with CRC and tests;
- GitHub Actions for format, clippy and Rust tests.

This is **software foundation only**. Real CSI acquisition, DSP, presence, respiration, localization and pose remain roadmap work until implemented and validated.

## What Echora will not claim

Until supported by reproducible evidence, the project will not claim:

- camera-grade pose from a single ESP32;
- medical-grade heart or respiration monitoring;
- reliable identity of a named person from RF;
- through-wall performance independent of material/environment;
- generalization from one room, one person or one capture session;
- real sensing when a pipeline is using replay or synthetic data;
- privacy merely because no visible camera is deployed.

## Reference instrumentation

Kinect/RGB-D/camera may be used during controlled data collection as a **reference/teacher**, never automatically as perfect ground truth. Calibration, synchronization, reference uncertainty and lineage remain part of the evidence record. Production inference is intended to be camera-free where the capability has been independently validated.

## Documentation

Start at [docs/README.md](docs/README.md). Important documents include:

- [Product requirements](docs/product/PRD.md)
- [System architecture](docs/architecture/ARCHITECTURE.md)
- [Repository structure](docs/engineering/STRUCTURE.md)
- [Sensing pipeline](docs/engineering/SENSING_PIPELINE.md)
- [Data and wire contracts](docs/engineering/DATA_AND_PROTOCOL.md)
- [Experimentation and scientific validation](docs/engineering/EXPERIMENTATION.md)
- [Test strategy](docs/quality/TEST_STRATEGY.md)
- [QA and assurance](docs/quality/QA_ASSURANCE.md)
- [Security and privacy](docs/security/SECURITY_PRIVACY.md)
- [SLO/SLA and performance budgets](docs/operations/SLO_SLA.md)
- [Roadmap](ROADMAP.md)

## License

A project license has **not yet been selected**. Code, datasets, trained weights and third-party assets may have different licensing constraints; no compatibility claim should be made until the licensing decision is recorded.

---

**Echora**: sense the space, preserve the evidence.
