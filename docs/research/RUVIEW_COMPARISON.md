# Echora vs RuView — Engineering Comparison

This comparison is intended to guide Echora design. It is not a criticism scorecard and must be updated when either project materially changes.

Reference:
- https://github.com/ruvnet/RuView
- https://cognitum.one/RuView

## High-level positioning

| Area | RuView | Echora direction |
|---|---|---|
| Goal | broad Wi-Fi/RF sensing platform with many integrated capabilities | evidence-governed RF spatial sensing instrument |
| ESP32 CSI | implemented | first hardware backend |
| Host stack | strong Rust-based pipeline | Rust modular runtime |
| UI | rich Three.js/observatory-style visualization | evidence-constrained Three.js spatial instrument |
| Simulation/replay | supported; project evolved toward clearer labeling | source mode is a core typed invariant from the beginning |
| Calibration | substantial | separate acquisition/environment/geometry/capability calibrations |
| Pose | present as research/runtime paths with varying maturity | withheld until topology/model/evidence gates pass |
| Vitals | experimental paths | respiration first; cardiac micro-motion separately gated |
| Point cloud | includes multimodal/reconstruction paths | camera, fused and RF-inferred representations are separate types |
| Scope | very broad | intentionally staged |
| Uncertainty | present in parts | mandatory spatial first-class object |
| Capability discovery | broad runtime behavior | explicit Capability Negotiator + renderer ceiling |
| Claims | has undergone important public corrections/retractions | claim/evidence matrix is authoritative from M0 |

## What Echora should learn from RuView

### Keep
- firmware/host separation;
- real CSI capture path;
- Rust systems runtime;
- WebSocket-driven live visualization;
- replayability;
- calibration;
- hardware testing;
- ADRs;
- model/evidence documentation;
- negative-result transparency.

### Avoid
- UI that can be mistaken for direct RF imaging;
- one generic "point cloud" label for different reconstruction sources;
- silent source substitution;
- metric names without locked definitions;
- promotion of one favorable benchmark into a universal capability claim;
- architecture expansion faster than validation capacity.

## Visualization difference

RuView demonstrates a visually compelling human-sensing dashboard.

Echora's distinctive rule is:

```text
runtime capability
+ evidence level
+ uncertainty
→ maximum render fidelity
```

Examples:
- CSI only → Signal View;
- link perturbation → RF link activity;
- spatial inference → occupancy field;
- localization → uncertain volume;
- track → centroid + covariance + trail;
- pose → skeleton only after gate;
- dense RF representation → only after explicit model/evidence support.

## Spatial geometry difference

Echora makes RF geometry visible:
- AP/transmitter positions;
- receiver-node positions/orientations;
- links;
- Fresnel-inspired support;
- per-link quality;
- spatial likelihood fields.

This is meant to help the operator understand *why* a location estimate exists instead of only seeing the final humanoid.

## "Why am I seeing this?"

Echora adds an inspectable explanation to rendered state:
- contributing nodes/links;
- source mode;
- evidence kind;
- confidence;
- spatial uncertainty;
- calibration;
- model;
- freshness;
- capability gate state;
- run/capture lineage.

This is useful for research, QA and debugging.

## Dense spatial outputs

Echora explicitly distinguishes:

```text
CAMERA_RECONSTRUCTED_POINT_CLOUD
MULTIMODAL_FUSED_POINT_CLOUD
RF_INFERRED_POINT_REPRESENTATION
```

A future RF-only dense reconstruction cannot inherit evidence from a camera/depth reconstruction.

## Engineering success criterion

"Looks like RuView" is not an acceptance criterion.

A successful Echora milestone must instead answer:
1. What was directly measured?
2. What was calibrated?
3. What was inferred?
4. What geometry supported it?
5. What uncertainty remains?
6. What test/experiment validates it?
7. Why was the UI allowed to render this representation?
