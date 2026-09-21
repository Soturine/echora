# Echora Web

## Status

**Designed, not yet implemented.**

Planned stack: TypeScript + Vite + Three.js.

## Visualization contract

The UI is not allowed to infer higher-fidelity state from lower-fidelity data.

Representation examples:

- presence only → occupancy indicator/blob;
- coarse location → uncertainty region/ellipse;
- tracked location → trajectory with confidence;
- pose unavailable → no skeleton;
- pose below gate → skeleton withheld;
- simulated/replay → persistent source badge;
- stale → persistent stale state;
- abstention → explicit reason, not numeric zero.

## Planned views

- system/node health;
- CSI/subcarrier diagnostics;
- calibration status;
- room/topology view;
- live sensing state;
- experiment/replay inspector;
- provenance/evidence drawer;
- SpatialTwin bridge view.

## QA invariants

Automated UI tests should prove:
- SIMULATED never renders as LIVE;
- replay is visibly replay;
- stale data cannot look current;
- unavailable pose cannot render a plausible skeleton;
- `null`/abstention is not converted to zero;
- confidence/uncertainty is inspectable.
