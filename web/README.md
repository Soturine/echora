# Echora Web

## Status

**Designed, not yet implemented.**

Planned stack: TypeScript + Vite + Three.js.

The authoritative UI design is documented in [Evidence-Constrained 3D Visualization Architecture](../docs/architecture/VISUALIZATION_ARCHITECTURE.md).

## Visualization contract

The UI is not allowed to infer higher-fidelity state from lower-fidelity data.

The backend supplies a capability state and renderer ceiling:

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

Representation examples:
- raw signal only → signal diagnostics, no person;
- presence/spatial likelihood → occupancy field/blob;
- coarse location → uncertainty volume;
- tracked location → trajectory + covariance;
- pose unavailable → no skeleton;
- pose below gate → skeleton withheld;
- simulated/replay → persistent source badge;
- stale → persistent stale state;
- abstention → explicit reason, not numeric zero.

## Planned views

### RF Lab
- room/world axes;
- AP/transmitter;
- sensing nodes;
- active links;
- Fresnel-inspired support volumes;
- link quality;
- calibration status.

### Signal View
- CSI amplitude/phase;
- subcarrier waterfalls;
- time/subcarrier surface;
- spectra / Doppler-like diagnostics;
- PCA/features;
- packet/timing health.

### Radar / Occupancy View
- spatial likelihood volume;
- uncertainty;
- link contributors;
- occupancy hypotheses.

### Track View
- centroid;
- covariance/uncertainty;
- trail;
- freshness;
- source links.

### Human / Pose View
Only after pose gates pass:
- keypoints/skeleton;
- per-joint confidence;
- uncertain/missing joints;
- model/evidence status.

### Reference View
Research-only:
- Kinect/RGB-D/camera reference;
- reference pose/point cloud;
- RF estimate;
- alignment error.

## "Why am I seeing this?"

Every rendered human/spatial object should expose:
- source mode;
- evidence kind;
- capability state;
- confidence;
- uncertainty;
- age/freshness;
- sensor/link contributors;
- calibration ID/state;
- model/version;
- run/capture ID;
- withholding/degradation reasons.

## Scene layers

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

Rendering components do not infer missing semantic state.

## QA invariants

Automated UI tests should prove:
- SIMULATED never renders as LIVE;
- replay is visibly replay;
- stale data cannot look current;
- unavailable pose cannot render a plausible skeleton;
- renderer never exceeds capability ceiling;
- `null`/abstention is not converted to zero;
- uncertainty is visible for spatial estimates;
- reference overlays are visibly distinct from RF inference;
- dense body / RF point representation cannot appear without corresponding capability state.
