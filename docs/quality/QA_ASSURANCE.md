# Echora QA and Assurance

QA in Echora covers requirement quality, architecture, code, data, experiments, hardware and claims.

## Assurance lenses

### Requirements
- clear and verifiable;
- no implementation accidentally masquerading as requirement;
- acceptance evidence defined for consequential features.

### Architecture
- dependency direction;
- ownership;
- protocol boundaries;
- failure/degraded behavior;
- security/privacy boundaries;
- calibration/evidence lifecycle.

### Code
- correctness;
- duplication;
- dead/unreachable behavior;
- unsafe defaults;
- error handling;
- concurrency;
- resource bounds;
- maintainability.

### Test quality
- behavior rather than implementation-only assertions;
- negative/boundary/degraded tests;
- realistic integration coverage;
- mocks not overrepresented;
- regression linkage.

### Experimental validity
- ground/reference truth;
- data leakage;
- split validity;
- metric definition;
- baseline;
- uncertainty;
- negative results;
- exact revision/hardware.

### Data quality
- schema validity;
- timestamps;
- missingness;
- capture completeness;
- duplicated windows;
- leakage;
- provenance;
- privacy.

### Operations
- observability;
- stale-data handling;
- restart/recovery;
- configuration drift;
- backup/restore if durable metadata becomes critical;
- SLO measurement.

## Finding states

Use precise states:
- observation;
- hypothesis;
- reproduced defect;
- confirmed security issue;
- experimental limitation;
- architecture risk;
- remediation implemented;
- retest passed/failed;
- accepted residual risk.

A scanner alert is not automatically a confirmed vulnerability. A code fix is not automatically a verified fix.

## Traceability

For meaningful capabilities, maintain the chain:

```text
need
→ requirement
→ decision
→ implementation
→ test
→ experiment / evidence
→ capability status
→ release claim
```

## Evidence grades

Project shorthand:
- **L0 SYNTHETIC** — simulation only;
- **L1 REPLAY** — deterministic captured/replayed software path;
- **L2 HIL** — target hardware behavior demonstrated;
- **L3 CONTROLLED** — controlled physical experiment with reference/controls;
- **L4 HELD-OUT** — held-out people/rooms/sessions as applicable;
- **L5 FIELD** — representative field validation;
- **L6 OPERATIONAL** — monitored production/operational evidence.

This ladder is an Echora project convention, not a universal scientific standard.

## Release claim gate

A README/release claim must not exceed the highest evidence level actually supporting that claim.

Example:
- packet decoder can be L1;
- ESP32 capture may be L2;
- presence may be L3/L4;
- pose may remain PROPOSED despite UI support.

## Independent review

For major milestones, perform an audit pass separate from the implementation pass:
- architecture review;
- claim/evidence review;
- security/privacy review;
- dataset split/leakage review;
- HIL/experimental protocol review.

## Gap register

Unresolved material gaps should be explicit in issues/docs rather than hidden in comments. Each gap should include:
- impact;
- evidence;
- owner;
- intended trigger/closure;
- residual limitation.
