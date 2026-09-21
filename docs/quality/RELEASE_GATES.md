# Echora Release Gates

A release gate is evidence for a transition, not a decorative checklist.

## Base software gate

- formatting/lint/type checks pass;
- unit/integration tests pass for supported build;
- parser/security boundary tests pass;
- dependency/security review completed;
- docs/changelog reflect the release;
- no known source-mode/evidence-label regression;
- artifact version matches tag.

## Firmware gate

- supported target builds;
- binary size budget checked;
- HIL smoke on declared hardware;
- provisioning/boot/stream test;
- protocol version compatible;
- known hardware limitations documented.

## Model gate

- immutable artifact hash;
- training/eval manifest;
- leakage checks;
- metric definition;
- held-out split;
- baseline;
- per-class/per-joint breakdown where relevant;
- model/runtime parity;
- confidence/rejection behavior;
- license/provenance.

## Capability claim gate

Before a capability appears as supported:
- requirement exists;
- implementation exists;
- tests exist;
- required hardware/experiment evidence exists;
- acceptance threshold met;
- failure modes documented;
- UI/API state accurately communicates maturity.

## Release blocking examples

Block release for:
- simulator shown as live;
- stale data shown as fresh;
- protocol parser panic on untrusted input;
- calibration mismatch silently accepted;
- model artifact mismatch;
- critical secret exposure;
- regression invalidating published evidence;
- unsupported claim added to README.

## Non-blocking experimental results

An experimental branch/result may ship when clearly labeled, isolated and not used to support a production capability claim.
