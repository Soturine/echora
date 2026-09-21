# Echora Data and Protocol Contracts

## Goals

- bounded parsing;
- versioned compatibility;
- replay parity;
- explicit provenance;
- no silent source substitution;
- deterministic units/endian/schema behavior.

## Source mode

Canonical source modes:

```text
LIVE
REPLAY
SIMULATED
```

A runtime must not auto-convert one source mode into another.

## Evidence kind

Canonical epistemic states:

```text
MEASURED
CALIBRATED
RECONSTRUCTED
INFERRED
GENERATED
SIMULATED
REFERENCE
```

Source mode and evidence kind are different dimensions. For example, a REPLAY frame can contain a previously MEASURED observation.

## Initial wire frame concept

The first sensor packet format should be compact and bounded.

```text
magic
protocol_version
message_type
header_len
payload_len
node_id
sequence
device_time_us
radio_metadata
subcarrier_count
flags
payload
crc32
```

Rules:
- fixed byte order;
- explicit maximum payload;
- exact version negotiation/unsupported-version error;
- length checked before allocation/use;
- CRC failure rejects the frame;
- unknown flags handled according to version rules;
- malformed packet never becomes a partial valid frame;
- monotonic sequence gaps are observable, not hidden.

## Frame identity

A decoded observation should be attributable to:

```text
run_id
capture_id
node_id
boot_id / timing_epoch
sequence
firmware_version
protocol_version
config_digest
source_mode
timestamp
```

## Calibration identity

Calibration objects should include:
- calibration ID;
- created time;
- environment ID;
- involved sensor IDs;
- topology/config digest;
- method/version;
- baseline statistics;
- quality/residuals;
- validity state;
- invalidation reason;
- originating run/capture IDs.

## Model identity

Inference records should reference:
- model ID/version;
- artifact hash;
- training dataset manifest;
- feature contract version;
- expected calibration/profile;
- runtime/provider version.

## Sensing result envelope

Conceptual contract:

```json
{
  "capability": "presence",
  "value": true,
  "confidence": 0.92,
  "source_mode": "LIVE",
  "evidence_kind": "INFERRED",
  "quality_flags": [],
  "run_id": "...",
  "capture_id": "...",
  "calibration_id": "...",
  "model_id": "...",
  "sensor_ids": ["node-01"],
  "observed_at": "...",
  "fresh_until": "...",
  "abstention": null
}
```

When abstaining:

```json
{
  "capability": "respiration_bpm",
  "value": null,
  "confidence": 0.0,
  "abstention": {
    "reason": "INSUFFICIENT_SNR"
  }
}
```

## Run manifest

Every experiment/benchmark should be able to emit a run manifest containing:
- git revision;
- dirty state;
- firmware version/hash;
- hardware IDs;
- OS/runtime;
- configuration digest;
- calibration;
- model artifacts;
- dataset/capture hashes;
- start/end times;
- source mode;
- random seeds;
- metric definitions;
- result artifact hashes.

## Dataset manifest

Dataset lineage should include:
- subject pseudonym/ID;
- room/environment;
- session;
- hardware/node;
- firmware;
- topology/layout;
- reference sensor;
- capture time;
- consent/purpose where applicable;
- preprocessing revision;
- split membership.

The project should not publish personally sensitive raw captures by accident. Dataset publication is a separate reviewed decision.

## Compatibility

- Additive fields are preferred within compatible versions.
- Semantic meaning changes require a version bump.
- A decoder must never guess incompatible payload semantics.
- Model feature-contract changes require explicit model/runtime compatibility checking.
