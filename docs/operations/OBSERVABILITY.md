# Echora Observability

Observability must answer both software and sensing questions.

## Runtime signals

### Logs
Structured events should include:
- timestamp;
- severity;
- component;
- run/capture ID;
- node ID where relevant;
- source mode;
- calibration/model IDs where relevant;
- error code/reason.

Do not log raw sensitive payloads by default.

### Metrics
Candidate metrics:
- frames received/dropped/out-of-order;
- CRC/decode failures;
- queue depth/high-water;
- bytes/sec;
- node last-seen;
- clock offset/drift estimates;
- calibration age/state;
- per-stage latency;
- model inference latency;
- abstention counts by reason;
- stale-result count;
- API/WS errors.

### Traces
Distributed tracing is not a baseline requirement. If the system becomes multi-process, trace IDs should bridge ingest → processing → output for latency/debug evidence.

## Sensing observability

Expose enough to distinguish:
- no person;
- no signal;
- stale sensor;
- invalid calibration;
- insufficient geometry;
- model unavailable;
- model abstention.

Those states must not collapse into the same boolean.

## Experiment/run identity

Operational metrics used as evidence must identify the actual revision, configuration, firmware, model and environment when those distinctions affect the claim.

## Dashboards

Future dashboard groups:
- node health;
- packet/timing quality;
- signal quality;
- calibration;
- capability outputs;
- confidence/abstention;
- host resource use.

Prometheus/Grafana may be adopted when the runtime warrants it; the data model should not depend on those products.
