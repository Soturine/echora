# Echora Roadmap

The roadmap is evidence-gated. A phase does not become complete because code exists; its acceptance evidence must exist too.

## M0 — Repository and evidence foundation

**Status:** active

Deliverables:
- product/architecture/stack documentation;
- evidence and provenance schema;
- source modes with fail-closed semantics;
- Rust workspace and core domain types;
- CI for formatting, linting and tests;
- baseline security/privacy policy;
- experiment and test templates.

Exit criteria:
- core schemas compile/validate;
- simulated/replay/live are structurally distinguishable;
- no capability claim implies hardware validation.

## M1 — Real CSI acquisition

Target:
- ESP32-S3 first; ESP32-C6 evaluated second;
- capture raw CSI + metadata without silent transformation;
- versioned binary wire protocol;
- packet sequence, timestamps, node identity and firmware identity;
- loss/jitter/drop metrics;
- deterministic recording and replay.

Acceptance:
- live capture demonstrated on physical hardware;
- replay reproduces decoded frame stream;
- malformed/fuzzed packets fail safely;
- no simulated source can be surfaced as LIVE.

## M2 — Single-link sensing

Target:
- baseline calibration;
- presence;
- coarse motion;
- respiration research path;
- quality and abstention.

Acceptance:
- physically empty-room controls;
- multiple distances/orientations;
- independent respiration reference;
- false positive/negative reporting;
- no heart-rate product claim.

## M3 — Multi-link spatial sensing

Target:
- 3–6 spatially separated RF links;
- time alignment;
- occupancy;
- coarse localization/tracking;
- geometry-aware uncertainty.

Acceptance:
- held-out positions;
- moved-node invalidation;
- known room coordinate reference;
- confidence calibration;
- degraded-node behavior.

## M4 — Reference capture and dataset pipeline

Target:
- synchronized CSI + Kinect/RGB-D/camera reference capture;
- calibration between RF/world/reference frames;
- dataset manifests and lineage;
- train/validation/test split tooling.

Acceptance:
- split by person, room, session, device/firmware and topology where applicable;
- no leakage through adjacent windows or shared sessions;
- reference uncertainty recorded;
- raw personal capture policy reviewed.

## M5 — Camera-free pose research

Target:
- CSI model training;
- pose/keypoint inference;
- calibrated uncertainty and abstention;
- temporal tracking.

Acceptance:
- metric definitions locked;
- held-out person/room/session results;
- baselines and ablations;
- failure modes visible;
- UI refuses skeleton rendering below evidence threshold.

## M6 — Cardiac micro-motion research

Target:
- heart-rate extraction only as an experimental module;
- compare against ECG/PPG/watch reference.

Acceptance:
- pre-registered metric/tolerance;
- multiple people/positions;
- false spectral peak analysis;
- capability remains EXPERIMENTAL unless robust gates pass.

## M7 — SpatialTwin integration

Target:
- place RF-derived human state in a metric reconstructed environment;
- integrate with Volumora/SpatialTwin through explicit contracts;
- preserve provenance from environment geometry and sensing separately.

Acceptance:
- coordinate transform chain verified;
- environment reconstruction confidence kept separate from human sensing confidence;
- replayable end-to-end run manifest.

## M8 — Optional multimodal RF adapters

Candidates:
- mmWave FMCW;
- UWB/CIR;
- BLE Channel Sounding;
- research NIC CSI;
- other future modalities.

Admission rule: capability and evidence first; technology novelty alone is insufficient.

## Non-goals until separately approved

- named-person identity from RF;
- medical diagnosis;
- covert surveillance deployment;
- claims of wall penetration independent of material/environment;
- cloud-first storage of raw human-sensing captures;
- microservices for architectural fashion.
