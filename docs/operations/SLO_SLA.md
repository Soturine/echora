# Echora SLO, SLA and Performance Budgets

## Status

Echora is currently a research/engineering project, not a hosted commercial service.

**No external contractual SLA exists today.**

This document defines internal SLO candidates and engineering budgets. They become enforceable only after measurement and explicit adoption.

## SLI families

### Sensor path
- frame acquisition rate;
- packet delivery ratio;
- out-of-order/duplicate rate;
- timestamp jitter;
- node heartbeat freshness.

### Runtime
- ingest latency;
- processing latency;
- queue depth;
- dropped frames;
- CPU/memory;
- restart/recovery.

### Sensing
- output freshness;
- abstention/rejection rate;
- confidence calibration;
- capability-specific accuracy/error metrics.

### API/UI
- request latency;
- WebSocket update freshness;
- stale-state detection;
- availability.

## Initial engineering budgets

These are **design targets, not measured claims**.

| SLI | Initial target |
|---|---|
| Host packet decode p95 | < 2 ms/frame |
| Deterministic DSP p95 | < 10 ms/window on reference dev host |
| Presence state end-to-end p95 | < 500 ms after sufficient window |
| UI state freshness | < 1 s for live presence/motion |
| Stale-node detection | < 5 s after expected heartbeat loss |
| Live→simulated silent fallback | 0 occurrences |
| Unlabeled source mode | 0 accepted external frames/results |
| Parser crash on malformed input | 0 tolerated |

Hardware-specific capture-rate targets will be adopted only after M1 measurements.

## Availability

For local research runtime:
- graceful failure is more important than a nominal uptime percentage;
- stale state must never masquerade as current state;
- a missing sensor should degrade/abstain rather than fabricate continuity.

Future service SLOs must specify:
- measurement window;
- excluded maintenance;
- dependency scope;
- region/deployment;
- error budget;
- response/recovery policy.

## RTO / RPO

Current research runtime:
- no contractual RTO/RPO;
- raw experiment capture integrity is protected by append/close semantics and checksums when implemented;
- experiment metadata should be reproducible from manifests.

Future durable deployments must define backup and restore validation before claiming RPO.

## Performance regression policy

A performance claim must record:
- hardware;
- OS;
- compiler/runtime;
- revision;
- workload/capture;
- repetitions;
- statistic.

CI on shared runners may compile/run smoke benchmarks but should not hard-fail on noisy timing thresholds without a controlled runner.

## Capacity planning

Before field deployment measure:
- nodes per host;
- frames/sec;
- bytes/sec;
- recording storage/day;
- CPU/core utilization;
- inference accelerator needs;
- WebSocket/client fan-out.

Do not size infrastructure from synthetic peak values alone.
