# Wi-Fi / RF Sensing Ecosystem Survey

This document maps projects, research systems and standards that influence Echora. It is not a feature checklist to copy. Each reference is evaluated by hardware assumptions, data path, validation method and transferability.

## Evaluation rule

For every external result:

```text
claim
→ source
→ hardware / RF topology
→ signal representation
→ dataset / environment
→ metric definition
→ validation boundary
→ reproducibility
→ transferability to Echora
→ decision
```

A demo video or README is evidence that a claim was made, not proof that the same capability transfers to Echora hardware.

## RuView

Reference:
- https://github.com/ruvnet/RuView
- https://cognitum.one/RuView

Relevant ideas:
- ESP32 CSI capture and transport;
- Rust host pipeline;
- WebSocket/Three.js visualization;
- live/replay/simulated source handling;
- calibration;
- pose/vital-sign research;
- point-cloud / multimodal visualization;
- extensive ADRs and proof-oriented documentation.

Important lessons:
- visualization can imply more spatial precision than a sensor actually supports;
- simulator fallback must never masquerade as live sensing;
- metric definitions and dataset composition can invalidate apparently strong results;
- capability maturity must be tracked independently from UI maturity;
- public corrections and negative results are useful evidence.

Echora decision:
- adopt strong provenance, replay, calibration and observability patterns;
- do not copy RuView capability claims;
- make uncertainty and source fidelity primary UI elements;
- prevent unsupported high-fidelity rendering by design.

## Espressif ESP-CSI / Wi-Fi sensing

Reference:
- https://github.com/espressif/esp-csi

Relevant ideas:
- official ESP32 CSI acquisition examples;
- router/sender/receiver experiment topologies;
- CSI capture and motion/presence research;
- hardware-family compatibility;
- newer Wi-Fi-sensing components and demos.

Echora decision:
- use Espressif examples as an upstream acquisition reference;
- pin exact ESP-IDF/chip versions in validation;
- preserve raw CSI instead of relying only on precomputed edge features;
- support separate raw-research and edge-sensing firmware profiles.

## ESP32-CSI-Tool

Reference:
- https://github.com/StevenMHernandez/ESP32-CSI-Tool

Relevant ideas:
- dataset-oriented CSI collection;
- active/passive experiment modes;
- serial/SD collection workflows;
- device-free localization research workflows.

Echora decision:
- provide import/compatibility tooling where useful;
- document collection topology explicitly in each capture manifest.

## ESPectre

Reference:
- https://github.com/francescopace/espectre

Relevant ideas:
- practical ESP32 Wi-Fi sensing productization;
- firmware + SDK separation;
- browser-oriented setup/monitoring;
- MQTT/home-automation integration;
- conservative capability communication.

Echora decision:
- borrow the distinction between reliable edge sensing and experimental research;
- make provisioning/health/OTA an engineering concern without moving experimental semantics into firmware too early.

## CSIKit

Reference:
- https://github.com/Gi-z/CSIKit

Relevant ideas:
- common processing across CSI sources from multiple hardware families.

Echora decision:
- define a hardware-neutral decoded CSI representation above device-specific adapters;
- keep chip-specific metadata rather than normalizing it away.

## SenseFi / Wi-Fi CSI benchmarks

Reference:
- https://github.com/xyanchen/WiFi-CSI-Sensing-Benchmark

Relevant ideas:
- repeatable activity-recognition baselines;
- dataset/model comparison;
- explicit evaluation protocols.

Echora decision:
- maintain simple statistical and classical-ML baselines before neural promotion;
- evaluate out-of-distribution boundaries, not only random frame splits.

## Widar / body-coordinate features

Reference family:
- Widar3.0 and related device-free activity recognition work.

Relevant idea:
- transform raw channel observations into representations intended to reduce environment/location dependence.

Echora decision:
- include physics-aware / geometry-aware feature families in ablations;
- do not assume end-to-end deep learning is the default best path.

## Respiration and micro-motion research

Reference family:
- FarSense and related CSI respiration research.

Relevant ideas:
- phase sensitivity;
- antenna/link diversity;
- periodic micro-motion extraction;
- importance of reference measurements and motion rejection.

Echora decision:
- respiration has a dedicated validation protocol;
- cardiac micro-motion remains a separate, harder capability;
- a plausible spectral peak is never sufficient evidence by itself.

## Wi-Fi pose research

Reference families:
- Person-in-WiFi;
- DensePose From WiFi;
- RF-Pose;
- multimodal Wi-Fi pose datasets.

Relevant ideas:
- camera/RGB-D supervision during training;
- RF-only inference after training;
- multi-antenna / multi-link geometry;
- keypoint and dense-body representations.

Echora decision:
- Kinect/RGB-D/camera may serve as a reference/teacher only in controlled collection;
- camera-free runtime claims require held-out people/rooms/sessions;
- UI skeleton rendering is evidence-gated.

## MM-Fi and multimodal benchmarks

Reference:
- MM-Fi multimodal sensing dataset/benchmark.

Relevant ideas:
- synchronized sensing modalities;
- shared reference frames;
- pose/action tasks;
- cross-modal supervision.

Echora decision:
- use public datasets as secondary benchmarks;
- never treat public benchmark performance as proof of Echora hardware performance;
- maintain separate `PUBLIC_DATASET` and `ECHORA_HARDWARE` evidence tracks.

## CSI-to-point-cloud research

Reference:
- CSI2PointCloud and related work.

Relevant idea:
- learned reconstruction of spatial point representations from CSI.

Echora decision:
- distinguish:
  - camera/depth reconstructed point clouds;
  - multimodal fused point clouds;
  - RF-inferred point clouds.
- never label all three simply "point cloud" without provenance.

## IEEE 802.11bf WLAN Sensing

Reference:
- IEEE 802.11bf WLAN Sensing standard.

Echora decision:
- current ESP32 CSI acquisition is an implementation backend, not the architectural boundary;
- future sensing adapters should map into a common observation model;
- do not claim 802.11bf compliance unless a specific hardware/software path is verified.

## Competitive position

RuView optimizes for broad capability integration and rapid end-to-end demonstrations.

Echora will optimize for:

```text
RF measurement fidelity
+ geometry
+ calibration
+ uncertainty
+ provenance
+ reproducible experimentation
+ evidence-constrained visualization
```

The project should be able to explain not only *what* it rendered, but *why the system was allowed to render it*.
