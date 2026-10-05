# Dataset and Benchmark Protocol

## Purpose

Wi-Fi sensing models are vulnerable to leakage and shortcut learning. Echora treats dataset design as part of verification.

## Evidence tracks

Keep results separate:

```text
PUBLIC_BENCHMARK
ECHORA_REPLAY
ECHORA_HARDWARE_CONTROLLED
ECHORA_HELD_OUT
ECHORA_FIELD
```

Performance on a public dataset does not validate Echora hardware.

## Dataset unit

Each sample/window must trace to:
- capture ID;
- subject/session;
- room/environment;
- hardware/node set;
- firmware;
- AP/router;
- topology;
- calibration;
- RF configuration;
- reference sensor;
- aligned time interval;
- preprocessing version.

## Split hierarchy

Prefer the strongest applicable split:

1. frame/window split — weakest, exploratory only;
2. session split;
3. day split;
4. person split;
5. room/environment split;
6. hardware/device split;
7. AP/router split;
8. topology/layout split;
9. combinations of the above.

Adjacent overlapping windows from one capture must not leak across train/test.

## Benchmark families

### Signal / acquisition
- frame rate;
- loss;
- jitter;
- coefficient validity;
- replay determinism.

### Presence
- precision;
- recall;
- F1;
- false positive/negative rate;
- time-to-detect;
- empty-room duration test.

### Motion/activity
- class metrics;
- confusion matrix;
- still-vs-motion false triggers;
- OOD rejection.

### Respiration
- MAE/median error;
- agreement plots/statistics where useful;
- rejection rate;
- motion-contaminated windows;
- distance/orientation breakdown.

### Localization
- median/p95 spatial error;
- uncertainty coverage;
- held-out grid positions;
- node-failure degradation.

### Pose
- PCK/MPJPE with exact normalization;
- per-joint error;
- visibility/occlusion breakdown;
- confidence calibration;
- abstention/coverage curve.

## Baselines

Every learned benchmark should include suitable lower-complexity baselines:
- chance/majority;
- threshold/statistical;
- logistic/SVM/tree/forest where applicable;
- simple temporal baseline;
- published/reference model where reproducible.

## Multiple comparisons

When many feature/model/hyperparameter variants are searched, keep the search process separate from final confirmation. Do not report the best explored run as if it were a frozen prospective result.

## Public datasets

Potential references include:
- activity-recognition CSI datasets;
- MM-Fi multimodal datasets;
- other properly licensed Wi-Fi sensing corpora.

For each public dataset record:
- license;
- hardware;
- sampling;
- task;
- split used;
- metric definition;
- known leakage/limitations.

## Echora benchmark manifest

A benchmark result should link:
- exact git revision;
- dataset manifest/hash;
- split manifest/hash;
- feature/model config;
- seeds;
- metric code version;
- calibration;
- result artifact;
- hardware/runtime for performance tests.

## Promotion

A capability claim moves from experimental to validated only when the benchmark matches the intended deployment boundary.
