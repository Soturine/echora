# Capability Negotiation

## Goal

Echora must know what the current hardware, topology, calibration, model and evidence actually permit.

A feature is not available merely because code for it exists.

## Inputs

The Capability Negotiator evaluates:
- source mode;
- sensor count/types;
- active RF links;
- band/channel/bandwidth;
- geometry availability;
- time-sync tier;
- calibration state;
- signal quality;
- required model/artifact;
- model compatibility;
- evidence/validation level;
- runtime health.

## Output states

Each capability returns one state:

```text
UNAVAILABLE
AVAILABLE
DEGRADED
EXPERIMENTAL
WITHHELD
```

with machine-readable reasons.

Example:

```json
{
  "capability": "pose",
  "state": "WITHHELD",
  "reasons": [
    "INSUFFICIENT_RF_GEOMETRY",
    "MODEL_NOT_VALIDATED_FOR_TOPOLOGY"
  ],
  "required_render_fidelity": "NONE"
}
```

## Example topology: one ESP32

Possible report:

```text
raw_csi            AVAILABLE
signal_view        AVAILABLE
motion             EXPERIMENTAL
presence           EXPERIMENTAL
respiration        EXPERIMENTAL / quality-gated
localization       UNAVAILABLE
multi_person       UNAVAILABLE
pose               UNAVAILABLE
rf_point_cloud     UNAVAILABLE
```

This is illustrative. Actual states are generated from validated capability rules.

## Example topology: calibrated multi-link

With several spatially known nodes:

```text
raw_csi            AVAILABLE
signal_view        AVAILABLE
presence           AVAILABLE/EXPERIMENTAL
motion             AVAILABLE/EXPERIMENTAL
occupancy_field    EXPERIMENTAL
localization       EXPERIMENTAL
tracking           EXPERIMENTAL
pose               WITHHELD until model/evidence gates pass
```

## Rules are versioned

Capability rules are configuration/code artifacts with:
- version;
- applicable hardware;
- minimum topology;
- calibration requirements;
- model requirements;
- evidence threshold;
- renderer ceiling.

A rule update that changes externally visible capability availability requires tests and changelog/ADR review when material.

## Renderer coupling

The negotiator provides a maximum visualization fidelity:

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

The frontend is not allowed to render above this ceiling.

## Evidence coupling

Capability status and scientific evidence are distinct:
- runtime capability says whether the system can attempt an output now;
- evidence level says how strongly that output has been validated.

A feature can therefore be:
- runtime `AVAILABLE`;
- evidence `L2 HIL`;
- product maturity `EXPERIMENTAL`.

All three should be inspectable.

## Why this is unique

This mechanism prevents a common sensing-demo failure: a polished UI continues to show a high-fidelity result when the topology or evidence can no longer support it.

Echora should fail by *reducing semantic fidelity*, not by inventing continuity.
