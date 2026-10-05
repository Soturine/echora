# RF Geometry and Fresnel Model

## Why this exists

A Wi-Fi CSI receiver does not directly output a camera-like 3D coordinate for a person. It observes changes in a radio channel produced by multipath propagation, reflection, diffraction and obstruction.

Echora's spatial view must therefore represent **where the measurements constrain a hypothesis**, not pretend every inference is a direct pixel/voxel observation.

## Coordinate geometry

Each radio endpoint is represented in the room frame by:

```text
position = (x, y, z)
orientation = quaternion / yaw-pitch-roll
position_uncertainty
orientation_uncertainty
antenna metadata
```

A link is the ordered relationship between transmitter and receiver plus RF configuration.

## Fresnel-inspired spatial support

For an idealized transmitter and receiver separated by distance `d`, loci of approximately equal excess path length form ellipsoidal regions. Fresnel-zone reasoning is useful as a visualization and feature prior for understanding where movement may strongly perturb a link.

Important limitation:

> A rendered Fresnel volume is a propagation model / spatial support prior, not a measured human volume.

Real indoor propagation includes multipath, walls, furniture, antenna patterns and dynamic interference.

## Link likelihood

Each link may contribute a bounded spatial field:

```text
L_i(x, y, z | observation, calibration, geometry)
```

The exact solver may be:
- analytical / Fresnel-weighted;
- tomographic;
- learned;
- hybrid.

Regardless of implementation, the output must retain:
- contributing link IDs;
- geometry revision;
- calibration ID;
- method/version;
- spatial uncertainty;
- validity domain.

## Fusion

A multi-link solver combines evidence into an occupancy/localization field.

Conceptually:

```text
link A field ─┐
link B field ─┼→ fusion → spatial likelihood → tracks
link C field ─┘
```

Fusion must not assume link independence unless justified.

## Representation hierarchy

Spatial outputs progress through increasing semantic commitment:

1. **RF link activity** — something changed on a link.
2. **occupancy field** — spatial probability/likelihood volume.
3. **localized object/person hypothesis** — centroid + covariance/volume.
4. **track** — temporal association of localized hypotheses.
5. **body extent** — coarse spatial body region.
6. **pose** — articulated keypoints.
7. **dense body / RF-inferred point representation** — highest research fidelity.

A lower level cannot be rendered as a higher level without an explicit estimator and evidence gate.

## Uncertainty

Preferred spatial forms:
- covariance ellipse/ellipsoid;
- credible/likelihood volume;
- particle cloud;
- grid/voxel distribution.

Avoid a visually exact point if the physical estimate is broad.

## Calibration dependency

Geometry is invalidated or degraded by changes such as:
- moving a node/AP;
- changing antenna orientation;
- channel/bandwidth/PHY changes;
- large room-layout changes;
- firmware changes that alter CSI interpretation;
- clock/timing changes for algorithms that depend on synchronization.

## Visualization rule

The 3D renderer may show:
- link endpoints;
- link/Fresnel support;
- occupancy likelihood;
- uncertainty;
- track history.

It must visually distinguish:
- measured RF geometry;
- modeled propagation/support;
- inferred human state.

## Future work

Potential spatial methods to benchmark:
- radio tomographic imaging;
- Fresnel-zone weighted localization;
- fingerprinting;
- particle filters;
- Bayesian occupancy grids;
- neural implicit fields;
- transformer-based multi-link fusion.

No method is selected as the default before M3 benchmarking.
