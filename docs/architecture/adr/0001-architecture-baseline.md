# ADR 0001 — Architecture baseline

- **Status:** Accepted
- **Date:** 2026-09-21

## Context

Echora requires embedded acquisition, deterministic signal processing, experimental ML workflows, a local API and spatial visualization. The design must preserve evidence fidelity and avoid copying the architecture of any single reference project.

## Decision

Use:
- ESP-IDF/C for sensor firmware;
- Rust for host runtime, protocols, DSP primitives, calibration/fusion and APIs;
- Python/PyTorch for offline training and research tooling;
- ONNX as the preferred first portable model interchange target when model export is suitable;
- TypeScript + Three.js for the research/operator UI;
- versioned binary packets for high-rate sensor transport;
- REST/WebSocket for control/state and MQTT only as an optional integration adapter;
- modular-monolith host deployment initially;
- raw CSI recording + deterministic replay as first-class architecture.

## Rationale

Rust provides strong boundary/type safety and deterministic systems performance. Python remains the strongest practical research ecosystem. TypeScript/Three.js supports spatial diagnostics without making the UI authoritative. A modular monolith minimizes distributed complexity during the experimental phase.

## Consequences

Positive:
- clear subsystem ownership;
- replay and HIL are easier;
- fewer hidden distributed failure modes;
- research stack remains productive.

Costs:
- multi-language toolchain;
- FFI/model interchange must be controlled;
- developers need Rust + ESP-IDF + Python + Node tooling for full-stack work.

## Rejected for now

- Python-only runtime;
- microservices from day one;
- Kafka as a default internal bus;
- database-first raw CSI pipeline;
- cloud-mandatory architecture;
- pose visualization before pose evidence.

## Revisit triggers

- measured host CPU bottleneck;
- need for remote multi-site gateways;
- GPU isolation;
- protocol requirements that UDP cannot meet;
- model runtime constraints;
- production availability target beyond a single host.
