# Claim and Evidence Matrix

This is the authoritative high-level view of what Echora may currently claim.

## Evidence ladder

- **L0 SYNTHETIC** — simulation only.
- **L1 REPLAY** — captured/replayable software path.
- **L2 HIL** — target hardware behavior demonstrated.
- **L3 CONTROLLED** — controlled physical experiment with controls/reference.
- **L4 HELD-OUT** — held-out people/rooms/sessions/topologies as applicable.
- **L5 FIELD** — representative field validation.
- **L6 OPERATIONAL** — monitored operational evidence.

## Current matrix

| Capability | Implementation status | Current evidence | Maximum allowed public claim |
|---|---|---:|---|
| evidence/source domain types | implemented | software unit tests | software contract exists |
| bounded v1 sensor protocol codec | implemented | software unit tests | codec foundation exists |
| real ESP32 CSI capture | not implemented | none | planned |
| deterministic capture/replay | not implemented | none | planned |
| CSI signal viewer | not implemented | none | designed |
| presence | not implemented | none | research target |
| motion | not implemented | none | research target |
| respiration | not implemented | none | experimental roadmap target |
| occupancy field | not implemented | none | research target |
| localization | not implemented | none | research target |
| tracking | not implemented | none | research target |
| pose | not implemented | none | future research target |
| heart rate | not implemented | none | future experimental research only |
| RF-inferred point representation | not implemented | none | exploratory future research |
| SpatialTwin integration | not implemented | none | future integration target |

## Claim format

When a capability becomes supported, its claim should identify:

```text
capability
implementation revision
evidence level
hardware/topology
environment/population
metric definition
dataset/split
uncertainty/rejection
limitations
```

## UI relationship

The UI may be complete before a sensing capability is validated. UI completeness must never promote this matrix.

Example:
- a skeleton component may be SOFTWARE_VERIFIED;
- pose sensing can still remain `not implemented / no evidence`.

## Update rule

This matrix changes only when new evidence exists for the exact capability. Roadmap intent alone cannot raise a row.
