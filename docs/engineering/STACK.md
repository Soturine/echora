# Echora Technical Stack

This is the initial stack decision, not a permanent technology mandate.

## Firmware

**Primary:** C + ESP-IDF

Initial hardware:
- ESP32-S3-DevKitC-1 class boards;
- ESP32-C6 as a research/secondary target after the S3 path is stable.

Responsibilities:
- CSI capture;
- bounded ring/buffer;
- metadata;
- transport;
- health;
- provisioning.

## Host runtime

**Primary:** Rust stable

Planned crates/modules:
- `echora-core` — evidence/domain types;
- `echora-protocol` — binary wire codec;
- `echora-ingest` — live/replay ingestion;
- `echora-signal` — DSP/features;
- `echora-calibration` — calibration lifecycle;
- `echora-inference` — runtime estimator interface;
- `echora-fusion` — multi-node geometry/fusion/tracking;
- `echora-recorder` — run/capture manifests;
- `echora-api` — REST/WebSocket;
- `echora-cli` — operator/research control plane.

Candidate Rust libraries will be chosen per capability after evaluation; no dependency is approved merely because another project uses it.

## ML / research

**Primary:** Python 3.12+ and PyTorch.

Use:
- training;
- notebook-free reproducible experiment scripts;
- dataset preparation;
- model evaluation;
- reference synchronization tooling;
- model export.

Preferred deployment boundary:
- train in PyTorch;
- export to ONNX where reliable;
- run host inference in Rust through a validated runtime or use an isolated Python research runner until parity is proven.

## UI

**Primary:** TypeScript + Vite + Three.js.

UI goals:
- source/evidence badges;
- RF node topology;
- signal diagnostics;
- uncertainty volumes;
- trajectory;
- skeleton only when pose inference meets rendering gates;
- experiment/run inspection.

## Storage

Early stages:
- append-only capture files for raw CSI;
- JSON/JSONL/CBOR/Parquet selected by artifact type;
- SQLite for local metadata if/when indexed querying becomes useful.

Do not introduce PostgreSQL/Redis by default. Add them only when multi-user, scale or deployment evidence requires them.

## Protocols

Sensor → host:
- UDP initially, versioned bounded binary frame;
- explicit sequence/CRC/source identity;
- no JSON on the high-rate hot path.

Host → UI:
- REST for control/config/status;
- WebSocket for streaming state.

Optional integrations:
- MQTT for home/IoT consumers.

## Observability

Application:
- structured logs;
- Prometheus-compatible metrics when runtime service begins;
- trace/run IDs.

Experimental:
- capture/run manifests;
- model/config/calibration hashes;
- stage latency and quality metrics.

Production-style tracing is added when cross-process boundaries justify it.

## Dev tooling

- Git/GitHub;
- GitHub Actions;
- rustfmt/clippy/cargo test;
- cargo-deny/cargo-audit or equivalent dependency controls;
- ruff/pytest/mypy where Python code exists;
- ESLint/TypeScript checks + Playwright when the UI exists;
- ESP-IDF build/test workflow;
- containerized host deployment later, not required for the first firmware milestone.

## Explicit non-defaults

Not default:
- Kubernetes;
- Kafka;
- service mesh;
- distributed tracing backend;
- vector database;
- LLM in the sensing decision path.

Any of these requires a capability-driven ADR.


## Hardware strategy refinement

The initial development target remains ESP32-S3, while C6 and C5 are explicit research follow-ons. Compatibility targets such as C3/classic ESP32 may be admitted for acquisition-lite roles when actual CSI APIs and throughput justify them.

Two firmware profiles are planned:
- `echora-node-raw` for raw CSI, dataset collection and algorithm research;
- `echora-node-edge` for validated operational sensing and integrations.

See `CSI_ACQUISITION_AND_HARDWARE_MATRIX.md`.

## Spatial visualization stack

Three.js remains the rendering foundation, but the web stack is organized around evidence-aware scene layers:
- environment;
- sensor topology;
- RF links/Fresnel support;
- occupancy field;
- tracks;
- pose;
- reference overlays;
- diagnostics;
- provenance/evidence overlay.

The renderer consumes capability status and cannot promote semantic detail independently.

## Benchmark and model governance

Python/PyTorch workflows must emit:
- dataset/split manifests;
- model cards;
- exact metric definitions;
- baseline comparisons;
- artifact hashes;
- held-out/OOD results;
- export/runtime parity evidence.

Public-dataset results and Echora-hardware results remain separate evidence tracks.

## Future sensing standards

The core observation model is intentionally vendor-neutral so future IEEE 802.11bf-capable adapters or other RF sensing backends do not require rewriting domain, calibration, evidence or visualization layers.
