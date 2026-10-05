# Evidence-Constrained 3D Visualization Architecture

## Objective

Echora's 3D UI should be visually strong without pretending Wi-Fi is a camera.

The renderer follows:

> **Visualization fidelity must not exceed sensing fidelity.**

The frontend receives typed state plus a maximum render fidelity from Capability Negotiation.

## Core views

### 1. RF Lab

Shows the physical sensing system:
- AP/transmitter;
- ESP32/RF nodes;
- node positions/orientations;
- active links;
- link quality;
- Fresnel-inspired support volumes;
- calibration state.

This view answers: **what RF geometry is producing the evidence?**

### 2. Signal View

Shows direct/derived signal data:
- CSI amplitude waterfall;
- phase waterfall;
- subcarrier-time surface;
- spectrogram/Doppler-like view;
- PCA/components;
- respiration spectrum;
- packet loss/timing quality.

This view answers: **what did the sensor measure?**

### 3. Radar / Occupancy View

Shows an inferred spatial likelihood field:
- voxel/grid/implicit volume;
- threshold contours;
- contributing links;
- uncertainty.

This is not a body mesh.

This view answers: **where does the current RF evidence support occupancy?**

### 4. Track View

Shows:
- track centroid;
- covariance/uncertainty;
- history/trail;
- age/freshness;
- contributing nodes;
- confidence.

This view answers: **how is the localized hypothesis moving over time?**

### 5. Human / Pose View

Only enabled after pose evidence gates pass.

Shows:
- skeleton/keypoints;
- joint-level confidence;
- uncertain/missing joints;
- temporal state;
- model/evidence badge.

Low-confidence joints are faded/withheld rather than rendered at full certainty.

### 6. Reference View

Controlled-research only.

Can overlay:
- Kinect/RGB-D/reference skeleton;
- camera/depth point cloud;
- RF estimate;
- alignment error.

It must clearly label reference data and must not exist in normal camera-free operation.

## Render fidelity levels

```text
NONE
SIGNAL_ONLY
OCCUPANCY_FIELD
LOCALIZED_VOLUME
TRACK
BODY_EXTENT
SKELETON
DENSE_BODY
RF_POINT_REPRESENTATION
```

The Capability Negotiator sets the ceiling.

## Epistemic renderer

The renderer must consume:
- source mode;
- evidence kind;
- capability state;
- confidence;
- uncertainty;
- freshness;
- calibration state;
- provenance.

Example behaviors:
- `SIMULATED` → permanent simulated badge/watermark;
- stale → freeze/fade + stale label, never silently animate;
- broad covariance → large translucent volume;
- insufficient geometry → occupancy only;
- pose withheld → no skeleton;
- uncertain joint → fade/hide joint;
- abstained vital → "withheld: reason", not zero.

## "Why am I seeing this?"

Every rendered spatial object should expose an inspection panel:

```text
object / capability
source mode
evidence kind
confidence
uncertainty
age
sensor/link contributors
calibration ID/state
model/version
run/capture ID
capability gate reasons
```

For research modes, add per-link diagnostics.

## Visual language

Preferred visual semantics:
- solid geometry = known physical hardware/environment;
- translucent field = modeled/inferred spatial support;
- trail = temporal inference;
- skeleton = validated articulated estimate;
- reference overlay = separate color/style and explicit REFERENCE badge.

Do not use graphics that imply millimeter precision without corresponding uncertainty evidence.

## Three.js architecture

Planned frontend layers:

```text
Scene shell
├── EnvironmentLayer
├── SensorTopologyLayer
├── RFLinkLayer
├── OccupancyFieldLayer
├── TrackLayer
├── PoseLayer
├── ReferenceLayer
├── DiagnosticsLayer
└── EvidenceOverlay
```

State comes from a normalized WebSocket/API model. Rendering components do not infer missing semantic state.

## Performance

Potential techniques:
- instanced rendering for voxel/particle fields;
- LOD for trails/fields;
- worker-based decoding for large diagnostic streams;
- decimated signal plots;
- GPU textures for heatmaps/volumes.

Performance optimization must not drop provenance or silently smooth away uncertainty.

## Comparison with RuView

RuView demonstrates a broad Three.js sensing dashboard and multimodal visualizations.

Echora differentiates by making:
- RF geometry visible;
- uncertainty first-class;
- capability negotiation explicit;
- source/evidence provenance inspectable;
- render fidelity mechanically gated.

The goal is not a prettier humanoid. The goal is a more truthful spatial instrument.
