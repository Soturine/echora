# Echora System Architecture

## Architectural style

Echora starts as a **modular monolith plus embedded firmware**, not a microservice system.

Why:
- low operational complexity;
- easier deterministic replay and debugging;
- fewer distributed-clock and deployment failure modes;
- preserves clear module boundaries without premature network boundaries.

Deployment can later split only where resource isolation, hardware placement or scale creates evidence for doing so.

## Context

```text
        ┌────────────────┐
        │ Wi-Fi AP / RF  │
        └───────┬────────┘
                │ channel perturbation
     ┌──────────▼──────────┐
     │ ESP32 sensing nodes │
     └──────────┬──────────┘
                │ versioned binary UDP (initial)
                ▼
┌───────────────────────────────────────────┐
│ Echora Host Runtime                       │
│                                           │
│  Ingest → Decode → Quality → DSP          │
│                ↓                          │
│           Calibration                     │
│                ↓                          │
│      Inference / Fusion / Tracking        │
│                ↓                          │
│         Spatial State Store               │
│                ↓                          │
│      API / WS / MQTT adapters             │
└───────────────┬───────────────────────────┘
                │
      ┌─────────┼──────────┐
      ▼         ▼          ▼
    Web UI    Recorder   SpatialTwin

Controlled reference path:
Kinect/RGB-D/Camera → Reference Adapter → synchronized run/dataset
```

## Module boundaries

### Firmware
Owns:
- Wi-Fi configuration;
- CSI callback/acquisition;
- bounded buffering;
- monotonic sequence;
- source timestamp where possible;
- node/firmware/protocol identity;
- basic health telemetry;
- transport.

Firmware must avoid heavy feature semantics in the first milestones so raw evidence remains available.

### Ingest
Owns:
- socket/transport;
- packet authentication when introduced;
- binary framing;
- CRC/checksum;
- length/range validation;
- source identity;
- rate/loss accounting;
- replay/live adapter parity.

### Core/domain
Owns:
- evidence types;
- source modes;
- run/capture identity;
- calibration identity;
- quality flags;
- timestamps and units;
- abstention semantics.

### DSP
Owns:
- CSI normalization;
- amplitude/phase derivation;
- outlier rejection;
- phase sanitization;
- subcarrier selection;
- temporal/spectral features;
- signal-quality metrics.

DSP should prefer explainable deterministic stages before adding learned replacements.

### Calibration
Owns:
- baseline/environment profiles;
- topology identity;
- validity/invalidation;
- calibration residual/quality;
- separation between baseline calibration and model training.

### Inference
Owns:
- presence/motion/respiration/localization/pose estimators;
- model/version identity;
- confidence;
- abstention;
- feature-to-output mapping.

### Fusion/tracking
Owns:
- multi-link alignment;
- geometry;
- state-level fusion;
- temporal filters;
- source conflict/degradation handling.

### Recording/replay
Owns:
- immutable run manifest;
- raw/derived capture lineage;
- deterministic replay;
- data checksum;
- model/config/calibration references.

### API
Owns:
- external versioned contracts;
- auth when remote exposure exists;
- backpressure/rate limits;
- state freshness.

### UI
Owns presentation only. It must not create inferred state not provided by the backend.

## Dependency direction

```text
adapters → application/runtime → domain/core
                      ↓
              dsp/inference/fusion
                      ↓
                    core
```

Core must not depend on hardware, web or UI frameworks.

## Trust boundaries

1. ESP32 / network packet boundary.
2. Capture/replay file boundary.
3. Model artifact boundary.
4. User configuration boundary.
5. Reference sensor/dataset import boundary.
6. Public API/WebSocket boundary if exposed outside localhost.
7. Browser/UI boundary.

Each boundary validates structure, bounds and provenance rather than trusting upstream intent.

## Failure semantics

- Packet malformed → reject + metric; never reinterpret.
- Live sensor unavailable → LIVE unavailable; never silently switch to SIMULATED.
- Calibration invalid/stale → inference withheld or explicitly degraded.
- Model missing/incompatible → typed capability unavailable.
- Reference stream missing → experiment marked incomplete, not fabricated.
- Low SNR/geometry → abstain or widen uncertainty.
- Node stale → exclude from fresh fusion.
- Clock discontinuity → new timing epoch / re-alignment.

## Data authority

```text
raw frame
→ decoded measurement
→ calibrated measurement/features
→ inferred state
→ fused/tracked state
→ visualization/automation
```

Each downstream layer derives from, but does not overwrite, upstream evidence.

## Future split points

Potential services only after evidence:
- hardware gateway on a separate edge machine;
- GPU model runtime;
- dataset/training infrastructure;
- MQTT/home automation bridge;
- long-term observability stack.

A split must document latency, failure, security and operational trade-offs in an ADR.
