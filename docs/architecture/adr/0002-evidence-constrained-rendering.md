# ADR 0002 — Evidence-constrained rendering

- **Status:** Accepted
- **Date:** 2026-10-05

## Context

RF sensing outputs vary greatly in semantic fidelity. A single link may support signal/motion evidence while a calibrated multi-link/model pipeline may support localization or pose. A generic 3D renderer can easily imply capabilities that do not exist.

## Decision

The backend owns a versioned **Capability Negotiator**. Every semantic capability has:
- runtime state;
- reasons;
- evidence level;
- maximum render fidelity.

The frontend may render at or below that fidelity but never above it.

Canonical render levels:

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

## Consequences

Positive:
- UI cannot silently promote presence into pose;
- degraded topology naturally reduces visualization fidelity;
- demos remain honest under partial failure;
- QA can contract-test renderer behavior.

Cost:
- backend/frontend contract is more explicit;
- capability rules require maintenance;
- visual transitions need deliberate UX.

## Security / privacy

The renderer does not reveal additional semantic state beyond what the backend explicitly authorizes. Reference-camera overlays are separate research-only layers.

## Verification

Required tests:
- simulated source cannot appear live;
- pose unavailable → skeleton impossible;
- stale calibration reduces/withholds calibrated capabilities;
- missing node reduces topology-dependent fidelity;
- null/abstained outputs cannot become numeric zero or fake body geometry.

## Revisit triggers

- first pose model;
- first RF-inferred dense representation;
- major SpatialTwin integration;
- new sensing modality requiring new render semantics.
