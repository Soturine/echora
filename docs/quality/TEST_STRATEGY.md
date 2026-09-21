# Echora Test Strategy

## Quality model

Testing spans software correctness and physical validity. They are not interchangeable.

```text
static checks
→ unit/property
→ integration/contract
→ replay/golden
→ system/E2E
→ SIL
→ HIL
→ experimental validation
→ field/operational validation
```

## Unit tests

Focus:
- binary codec;
- checksums;
- sequence arithmetic;
- timestamp conversion;
- units;
- DSP primitives;
- filter behavior;
- quality gates;
- calibration state machine;
- confidence/abstention rules.

## Property/fuzz tests

High-value targets:
- packet decoder;
- file/capture parser;
- configuration parser;
- numeric DSP boundaries;
- non-finite values;
- malformed arrays/subcarrier counts;
- integer overflow/length calculations.

Properties:
- parser never panics on arbitrary input;
- encode→decode round-trip;
- invalid lengths never allocate beyond configured bound;
- source/evidence enum cannot alias;
- replay preserves frame identity.

## Golden/replay tests

Maintain small, distributable, privacy-safe fixtures:
- deterministic synthetic CSI;
- sanitized captured fixtures if publication is allowed;
- malformed packets;
- packet loss/out-of-order scenarios;
- timing discontinuity scenarios.

Golden tests must pin only stable semantics, not accidental serialization formatting.

## Integration tests

Cover:
- UDP ingest → decoded observation;
- recorder → replay → same decoded stream;
- calibration → feature extraction;
- inference adapter → typed result envelope;
- API freshness/degraded-state behavior.

## Firmware tests

Layers:
- host-side pure-C/unit tests where possible;
- QEMU/simulation for boot/config logic where meaningful;
- target compile matrix;
- HIL on ESP32-S3;
- later ESP32-C6.

HIL scenarios:
- boot/provision;
- valid CSI stream;
- disconnect/reconnect;
- AP loss;
- target host unavailable;
- sequence continuity;
- buffer pressure;
- reboot/timing epoch;
- firmware upgrade/rollback when OTA exists.

## Experimental validation tests

Presence:
- empty/occupied;
- still/moving;
- multiple distances/orientations;
- furniture/environment changes.

Respiration:
- controlled breathing rates;
- natural breathing;
- motion contamination;
- reference sensor comparison.

Localization:
- known grid positions;
- held-out positions;
- node removal;
- moved-node invalidation.

Pose:
- held-out subjects/rooms/sessions;
- per-joint metrics;
- reference occlusion;
- confidence/rejection calibration.

## Performance testing

Measure exact hardware/revision:
- firmware capture rate;
- packet loss;
- ingest throughput;
- DSP latency;
- inference latency;
- memory;
- CPU;
- end-to-end freshness.

Use p50/p95/p99 where distributions matter. No universal threshold is claimed until measured.

## Security tests

- dependency/advisory scan;
- secret scan;
- malformed network inputs;
- auth/authorization once non-local API is enabled;
- path traversal for capture/model import;
- unsafe deserialization;
- DoS/resource bounds;
- artifact checksum/signature verification when added.

## UI tests

- source mode visibly correct;
- SIMULATED cannot appear LIVE;
- stale state visibly stale;
- abstention not rendered as zero;
- confidence thresholds affect representation;
- no pose skeleton when backend capability is unavailable;
- keyboard/accessibility basics.

## Regression rule

Every confirmed defect should get the narrowest useful regression test at the layer where it can be deterministically reproduced.

## Test evidence

Store test reports/artifacts in CI where useful, but do not commit transient bulky outputs. Material experimental evidence belongs in versioned experiment/result records or referenced immutable artifacts.
