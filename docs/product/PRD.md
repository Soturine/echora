# Echora Product Requirements Document

## 1. Problem

Most human sensing relies on visible cameras, wearables or dedicated radar hardware. Commodity Wi-Fi already interacts continuously with occupants and environments, and CSI can expose radio-channel perturbations. The engineering challenge is to convert those perturbations into useful, bounded, reproducible sensing outputs without overstating what the signal contains.

## 2. Product vision

Echora will be an open platform for device-free human sensing using Wi-Fi CSI first and additional RF modalities later. It should support research, reproducible experiments and eventually local automation/spatial applications while making uncertainty and evidence visible.

## 3. Users

Primary:
- developer/researcher building RF sensing experiments;
- student/engineer evaluating CSI hardware and algorithms;
- operator visualizing a controlled sensing deployment.

Future:
- smart-home / assisted-living integrations;
- SpatialTwin applications;
- industrial occupancy and non-visual sensing.

## 4. Core jobs

1. Capture real CSI reliably from known hardware.
2. Record and replay captures deterministically.
3. Calibrate environment and sensor topology.
4. Extract signal features with inspectable DSP.
5. Estimate bounded human state with confidence/abstention.
6. Validate claims against independent references.
7. Visualize only what the current evidence supports.
8. Preserve lineage from raw observation to displayed result.

## 5. Functional requirements

### FR-001 — Source identity
Every frame shall identify source mode, sensor/node identity, firmware/protocol version and acquisition timestamp.

### FR-002 — Source-mode separation
LIVE, REPLAY and SIMULATED sources shall be distinct and must never silently fall back into one another.

### FR-003 — Raw capture
The system shall support preserving raw CSI and acquisition metadata for reproducible offline processing where configured.

### FR-004 — Replay
A recorded capture shall be replayable through the same decoding/processing contracts used by live ingestion.

### FR-005 — Calibration
Calibrations shall have stable identity, environment/sensor topology context, timestamps and explicit invalidation triggers.

### FR-006 — Quality
Inference outputs shall expose confidence/quality flags and may abstain.

### FR-007 — Presence/motion
The early product shall support research-grade presence and coarse motion estimation from a calibrated RF link.

### FR-008 — Respiration
Respiration extraction may be enabled as an experimental feature only with confidence and validation metadata.

### FR-009 — Localization
Multi-node deployments may estimate coarse room position/track with explicit uncertainty.

### FR-010 — Pose
Pose is future research scope and shall not be represented as camera-equivalent without held-out validation.

### FR-011 — Reference capture
The system shall support synchronized reference instrumentation such as Kinect/RGB-D/camera for controlled dataset creation.

### FR-012 — API
Processed state shall be available through versioned machine-readable interfaces.

### FR-013 — Visualization
The UI shall render source mode, confidence and degraded/abstained states prominently.

### FR-014 — Experiment records
Material benchmark/model claims shall link to an experiment/run record and exact revision.

## 6. Quality requirements

### QR-001 Correctness
Parsing, units, timestamps, sequence handling and evidence semantics are deterministic and contract-tested.

### QR-002 Reproducibility
Replayable runs pin code/model/config/calibration/data identities.

### QR-003 Reliability
Transient node/network failure does not convert stale data into fresh inference.

### QR-004 Performance
Latency and throughput have explicit budgets; optimizations require measurement.

### QR-005 Security
Untrusted packets/configuration are validated at boundaries. Secrets are not embedded in firmware images or repository history.

### QR-006 Privacy
Raw/derived human sensing data is treated as sensitive even without cameras.

### QR-007 Maintainability
Subsystem ownership and dependency direction remain explicit; algorithms are replaceable behind stable domain contracts.

### QR-008 Portability
The core remains deployable on a developer workstation and Linux edge host; firmware targets are isolated from host runtime code.

### QR-009 Observability
Frame loss, clock behavior, node health, calibration state, inference quality and runtime errors are observable.

## 7. Non-goals for initial releases

- medical diagnosis;
- identity recognition;
- law-enforcement/covert-surveillance productization;
- cloud-scale microservice architecture;
- universal through-wall guarantees;
- camera-grade pose from a single SISO node;
- automatic claims based on synthetic-only results.

## 8. Success criteria

The first meaningful success is not a polished humanoid visualization. It is:

- real CSI capture from supported ESP32 hardware;
- deterministic replay;
- measured packet/timing quality;
- reproducible empty-vs-occupied experiment;
- bounded presence/motion output;
- clear failure/uncertainty behavior;
- evidence that another engineer can reproduce.

## 9. Claim policy

Public capability claims must state:
- capability;
- evidence state;
- hardware/topology;
- environment/population;
- metric definition;
- dataset/split;
- relevant limitations.

A claim without those details is marketing, not acceptance evidence.
