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
- experiment, model, dataset, calibration and hardware-validation templates;
- RF geometry, coordinate-frame, calibration and capability-negotiation specifications;
- claim/evidence matrix and ecosystem survey.

Exit criteria:
- core schemas compile/validate;
- simulated/replay/live are structurally distinguishable;
- topology/calibration/spatial/capability contracts are defined;
- no capability claim implies hardware validation;
- documentation index and traceability are consistent.

## M1 — Real CSI acquisition

Target:
- ESP32-S3 first;
- ESP32-C6 and ESP32-C5 evaluated as follow-on RF targets;
- separate raw-research and edge-sensing firmware profiles;
- capture raw CSI + metadata without silent transformation;
- versioned binary wire protocol;
- packet sequence, timestamps, node identity and firmware identity;
- loss/jitter/drop metrics;
- deterministic recording and replay;
- hardware-neutral decoded observation model above device-specific adapters.

Acceptance:
- live capture demonstrated on physical hardware;
- replay reproduces decoded frame stream;
- malformed/fuzzed packets fail safely;
- no simulated source can be surfaced as LIVE;
- hardware validation record exists;
- acquisition quality is characterized for the supported target.

## M2 — Single-link sensing

Target:
- acquisition/environment calibration;
- drift detection foundation;
- presence;
- coarse motion;
- respiration research path;
- quality and abstention;
- signal-view diagnostics.

Acceptance:
- physically empty-room controls;
- multiple distances/orientations;
- independent respiration reference;
- false positive/negative reporting;
- stale/invalid calibration behavior verified;
- no heart-rate product claim.

## M3 — Multi-link spatial sensing

Target:
- 3–6 spatially separated RF links;
- coordinate-frame registry;
- time alignment;
- RF/Fresnel geometry view;
- occupancy likelihood field;
- coarse localization/tracking;
- geometry-aware uncertainty;
- capability negotiation based on topology.

Acceptance:
- held-out positions;
- moved-node invalidation;
- known room coordinate reference;
- confidence/uncertainty calibration;
- degraded-node behavior;
- UI render fidelity cannot exceed capability ceiling.

## M4 — Reference capture and dataset pipeline

Target:
- synchronized CSI + Kinect/RGB-D/camera reference capture;
- calibration between RF/world/reference frames;
- dataset manifests and lineage;
- train/validation/test split tooling;
- public benchmark adapters;
- model/data cards.

Acceptance:
- split by person, room, session, device/firmware and topology where applicable;
- no leakage through adjacent windows or shared sessions;
- reference uncertainty recorded;
- raw personal capture policy reviewed;
- benchmark manifests identify public vs Echora-hardware evidence tracks.

## M5 — Camera-free pose research

Target:
- CSI model training;
- pose/keypoint inference;
- calibrated uncertainty and abstention;
- temporal tracking;
- evidence-constrained skeleton rendering.

Acceptance:
- metric definitions locked;
- held-out person/room/session results;
- baselines and ablations;
- failure modes visible;
- UI refuses skeleton rendering below evidence threshold;
- per-joint confidence and rejection are available;
- claim/evidence matrix is updated only after acceptance.

## M6 — Cardiac micro-motion research

Target:
- heart-rate extraction only as an experimental module;
- compare against ECG/PPG/watch reference.

Acceptance:
- pre-registered metric/tolerance;
- multiple people/positions;
- false spectral peak analysis;
- rejection/abstention behavior;
- capability remains EXPERIMENTAL unless robust gates pass.

## M7 — SpatialTwin integration

Target:
- place RF-derived human state in a metric reconstructed environment;
- integrate with Volumora/SpatialTwin through explicit contracts;
- preserve provenance from environment geometry and sensing separately;
- expose transform-chain uncertainty.

Acceptance:
- coordinate transform chain verified;
- environment reconstruction confidence kept separate from human sensing confidence;
- replayable end-to-end run manifest;
- SpatialTwin rendering respects Echora's renderer ceiling.

## M8 — Optional multimodal RF adapters

Candidates:
- IEEE 802.11bf-capable WLAN sensing hardware;
- mmWave FMCW;
- UWB/CIR;
- BLE Channel Sounding;
- research NIC CSI;
- other future modalities.

Admission rule: capability and evidence first; technology novelty alone is insufficient.

## M9 — RF-inferred dense spatial research

Exploratory targets:
- dense body representations;
- RF-inferred point representations;
- neural implicit occupancy/body fields;
- multimodal comparison against depth/reference data.

Acceptance:
- explicit distinction between camera/depth reconstruction, multimodal fusion and RF-only inference;
- held-out evaluation;
- uncertainty representation;
- no dense renderer without evidence gate.

## Non-goals until separately approved

- named-person identity from RF;
- medical diagnosis;
- covert surveillance deployment;
- claims of wall penetration independent of material/environment;
- cloud-first storage of raw human-sensing captures;
- microservices for architectural fashion;
- 802.11bf compliance claims without verified compatible hardware/software.
