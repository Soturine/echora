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
