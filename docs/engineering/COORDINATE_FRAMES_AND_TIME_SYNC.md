# Coordinate Frames and Time Synchronization

Spatial sensing fails quietly when coordinate and time semantics are vague. Echora treats both as first-class contracts.

## Frame tree

Minimum frame hierarchy:

```text
world
└── room/<environment_id>
    ├── rf/ap/<id>
    ├── rf/node/<id>
    ├── reference/<device_id>
    └── spatial/person/<track_id>
```

Optional frames:
- camera optical frame;
- Kinect depth/color frames;
- SpatialTwin/model frame;
- calibration fixture frame.

## Transform contract

A transform includes:
- parent frame;
- child frame;
- translation;
- rotation;
- transform uncertainty;
- calibration ID;
- valid-from / valid-until;
- source/evidence kind.

Never silently use a transform from a different room/topology revision.

## Units

Canonical:
- distance: meters;
- angles: radians internally;
- timestamps: integer nanoseconds/microseconds or explicit UTC/monotonic domains;
- rates: SI-derived units.

Display units may vary; stored contracts do not.

## Time domains

Track at least:

```text
device monotonic time
host receive monotonic time
wall-clock UTC
aligned experiment time
reference sensor time
```

These are not interchangeable.

## Clock model

For each node/reference stream, estimate when needed:
- offset;
- drift;
- alignment uncertainty;
- timing epoch / reboot identity.

A device reboot creates a new timing epoch even if node ID is unchanged.

## Synchronization tiers

### T0 — arrival-only
Host receive timestamps only.

Suitable:
- basic single-link visualization;
- coarse presence/motion.

Not sufficient evidence for precise multi-node fusion.

### T1 — device timestamp + host alignment
Device monotonic timestamps with offset/drift estimation.

Suitable:
- improved multi-link window alignment;
- many research experiments.

### T2 — synchronized reference
RF stream aligned to Kinect/RGB-D/PPG/ECG reference with measured error.

Required:
- supervised pose/vital-sign validation.

### T3 — hardware-assisted synchronization
External/common timing where the selected hardware supports it.

Research/future.

## Data-window policy

Every fused or learned sample records:
- start/end aligned time;
- contributing frame ranges;
- missing/gap mask;
- resampling/interpolation policy;
- maximum alignment error.

No silent interpolation across large gaps.

## Reference alignment

For reference sensors:
- estimate latency, not only nominal FPS;
- record dropped frames;
- expose visibility/confidence;
- keep reference timestamps and aligned timestamps;
- measure sync error with a repeatable event/fixture when practical.

## SpatialTwin integration

SpatialTwin transforms must be composed explicitly:

```text
RF node frame
→ room frame
→ SpatialTwin world frame
```

Environment reconstruction uncertainty and RF human-state uncertainty remain separate and are combined only when required by a specific output.
