# Repository Structure and Ownership

```text
echora/
├── .github/
│   └── workflows/            CI/CD gates
├── crates/
│   ├── echora-core/          domain/evidence contracts
│   └── echora-protocol/      sensor wire codec
├── docs/
│   ├── architecture/
│   ├── engineering/
│   ├── operations/
│   ├── product/
│   ├── quality/
│   ├── research/
│   ├── risk/
│   └── security/
├── firmware/                 ESP-IDF target code
├── hardware/                 BOM/topology/calibration setup
├── ml/                       offline training/evaluation
├── schemas/                  machine-readable public contracts
├── templates/                ADR/test/experiment templates
├── tests/                    cross-layer harnesses
└── web/                      operator/research UI
```

## Planned Rust module ownership

As implementation grows, prefer:

```text
echora-core
  ↑
echora-protocol
  ↑
echora-ingest ─── echora-recorder
  ↓
echora-signal
  ↓
echora-calibration
  ↓
echora-inference
  ↓
echora-fusion
  ↓
echora-api / echora-cli
```

Exact dependencies may differ, but domain/evidence types must stay independent from presentation and hardware frameworks.

## Code placement rules

- generic domain invariant → `echora-core`;
- packet/file codec → protocol/recorder;
- physical signal algorithm → signal;
- environment/topology validity → calibration;
- model invocation → inference;
- multi-source state combination → fusion;
- network/UI adaptation → API/web;
- target-specific acquisition → firmware/adapter.

Avoid “utils” as a dumping ground. Shared code needs a named responsibility.

## Test placement

- module unit tests beside implementation;
- crate integration tests under crate `tests/`;
- system/replay/HIL orchestration under root `tests/`;
- scientific experiment scripts under `ml/` or future `experiments/` with run manifests.

## Documentation ownership

One concept should have one authoritative owner. Other docs link to it rather than duplicating large normative blocks.
