# AGENTS.md — Echora engineering contract

## Mission

Echora is a device-free RF sensing research/engineering platform. The repository must preserve a strict boundary between what is proposed, implemented, tested, integrated, hardware-verified and validated.

The engineering reference is Derivanta's Engineering Constitution and applicable domain guidance, especially perception/spatial sensing, experimental engineering, evidence, test planning, configuration, security/privacy, operational acceptance and document authority.

## Authority order

1. safety/legal/authorization boundaries;
2. explicit current task requirements;
3. approved Echora requirements, ADRs and contracts;
4. Engineering Constitution / applicable Derivanta guidance;
5. current architecture and schemas;
6. implementation conventions;
7. agent/tool preference.

Never silently weaken evidence integrity, privacy, security or experimental validity.

## Mandatory epistemic rules

Never collapse these states:

```text
planned ≠ implemented
implemented ≠ tested
tested ≠ integrated
integrated ≠ hardware-verified
hardware-verified ≠ validated
measured ≠ reconstructed ≠ inferred ≠ generated ≠ simulated
reference sensor ≠ perfect ground truth
replay ≠ live
synthetic ≠ real
one-room result ≠ generalization
```

A dashboard must not visually imply more certainty than the evidence supports.

## Design defaults

- Modular monolith first; split deployment only with measured need.
- Rust owns ingestion, protocol validation, deterministic DSP/runtime state and safety-critical evidence boundaries.
- C/ESP-IDF owns firmware.
- Python owns offline training/research workflows where ecosystem leverage matters.
- Web UI is an observer of typed backend state, not an authority.
- Raw CSI and capture metadata are first-class evidence artifacts.
- Every model output carries confidence/quality/provenance and can abstain.
- Simulation and replay require explicit opt-in and visible source labels.
- No silent fallback from live to simulated data.
- Keep secrets out of source control.
- Prefer local/offline processing for human-sensing data.

## Required change behavior

For meaningful changes:

1. identify the authoritative requirement/ADR/contract;
2. inspect related code/tests/docs before editing;
3. update the smallest coherent boundary;
4. add or update verification at the same layer;
5. consider negative/degraded/failure paths;
6. assess evidence invalidation (model, calibration, hardware, environment);
7. update docs/changelog/roadmap when public behavior changes;
8. report what was actually run and what remains unverified.

## ADR triggers

Create/update an ADR when changing:

- subsystem ownership or dependency direction;
- wire/data schema compatibility;
- source/evidence semantics;
- raw-data retention or privacy boundary;
- calibration identity or invalidation policy;
- model/runtime format;
- public API;
- security/authentication boundary;
- sensor topology assumptions;
- metric definitions used for project claims;
- deployment architecture.

## Testing expectations

Use the relevant subset:

- unit/property tests for parsers, math and invariants;
- golden/replay tests for deterministic signal paths;
- integration/contract tests for service boundaries;
- hardware-in-the-loop tests for ESP32 behavior;
- experimental validation against independent references;
- cross-room/person/session/device splits for ML claims;
- soak, loss, jitter, restart and degraded-network tests;
- security and dependency scanning;
- performance budgets tied to exact hardware/revision.

Mocks prove software behavior only. They do not prove physical sensing.

## Definition of Done

A change is done only when the relevant subset is satisfied:

- requirement/decision traceable;
- implementation complete;
- tests and evidence updated;
- error/degraded paths handled;
- security/privacy impact assessed;
- compatibility/migration considered;
- observability included when operationally relevant;
- docs reflect current state;
- no unsupported claim was introduced;
- unresolved gaps are explicit.

## Capability labels

Use these labels in docs/issues/reports where useful:

```text
PROPOSED
DESIGNED
IMPLEMENTED
SOFTWARE_VERIFIED
REPLAY_VERIFIED
HARDWARE_VERIFIED
EXPERIMENTALLY_VALIDATED
OPERATIONALLY_VALIDATED
DEPRECATED
REJECTED
```

A higher label requires evidence for that exact revision, hardware and environment.
