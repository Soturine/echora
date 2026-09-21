# Echora Cross-Layer Tests

This directory is reserved for tests that cross crate/adapter boundaries.

Planned suites:
- protocol fixtures;
- ingest/replay parity;
- malformed packet corpus;
- deterministic DSP fixtures;
- simulated timing/loss scenarios;
- API/UI contract;
- firmware HIL orchestration;
- experiment acceptance harness.

Unit tests stay beside their code. Physical experiment data is not committed here unless it is small, sanitized and explicitly approved.
