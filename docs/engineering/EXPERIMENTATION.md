# Echora Experimental Engineering

RF sensing claims depend on physical experiments. The experiment is therefore an engineering artifact, not an informal demo.

## Required experiment record

Material experiments should state:

1. question/hypothesis;
2. capability under test;
3. exact code/firmware/model revisions;
4. hardware and topology;
5. room/environment;
6. calibration;
7. subjects/sessions;
8. reference instrumentation;
9. controlled variables;
10. uncontrolled/confounding variables;
11. input/capture IDs;
12. preprocessing;
13. metric definitions;
14. sample/window policy;
15. seeds/repetitions;
16. exclusions;
17. results and uncertainty;
18. failures/negative results;
19. limitations;
20. scope of the supported claim.

## Baselines and controls

Examples:
- physically empty room;
- static room with no person;
- known periodic mechanical disturbance;
- occupied still;
- occupied moving;
- simulated deterministic signal;
- simple statistical baseline;
- no-calibration baseline;
- single-link vs multi-link.

## Ground/reference truth

Reference devices are not automatically perfect.

For Kinect/RGB-D/camera:
- record frame rate/resolution;
- intrinsic/extrinsic calibration;
- pose model/version if using MediaPipe/etc.;
- confidence/visibility;
- synchronization error;
- occlusion and missing joints.

For breathing/heart:
- record device/model;
- sampling interval;
- placement;
- time synchronization;
- algorithm/export mode;
- known reference limitations.

## Split policy

For learned human sensing, random frame splitting is insufficient by default.

Evaluate at the strongest applicable boundaries:
- person;
- room;
- session/day;
- node/device;
- firmware;
- AP/router;
- topology;
- furniture/layout;
- orientation/distance.

Adjacent windows from the same capture must not leak across train/test unless the experiment explicitly studies that weaker condition.

## Metrics

Metric definitions are part of the claim.

Examples:
- presence: precision/recall/F1, false positive rate, false negative rate, time-to-detect;
- respiration: MAE, median absolute error, Bland–Altman style agreement when useful, rejection rate;
- localization: median/95th percentile position error, coverage at radius, calibration of spatial uncertainty;
- pose: MPJPE/PCK with explicit normalization, per-joint breakdown, coverage/rejection;
- timing: p50/p95/p99 latency and jitter;
- packet path: loss/out-of-order/duplication.

Never publish a metric name without its operational definition when different definitions materially change the result.

## Uncertainty and negative results

Report:
- confidence intervals/bootstrap where appropriate;
- run-to-run variance;
- rejected/abstained samples;
- failed captures;
- environmental sensitivity;
- known confounders.

Negative results remain valuable. Do not delete them merely because a new approach performs better.

## Promotion ladder

```text
synthetic demonstration
→ deterministic replay
→ controlled hardware experiment
→ held-out controlled validation
→ cross-person/room/session validation
→ field validation
→ operational acceptance
```

Promotion to a higher rung requires evidence at that rung.

## Reproducibility

Prefer scripts over manual notebook state. Notebooks may explore; canonical results should be regenerable from versioned commands/configuration.

Every headline result should ideally be reproducible through:
- a run manifest;
- capture/dataset reference;
- command;
- configuration;
- metric implementation;
- result artifact.

## Claim review

Before README/public claim promotion, ask:
- Does the metric actually measure this claim?
- Is the evaluation set independent?
- Did a simpler baseline achieve the same result?
- Is this one room/person/session?
- Is source data live or simulated?
- Did calibration or reference data leak target information?
- Does the UI imply greater resolution than measured?
