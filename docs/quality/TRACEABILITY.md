# Echora Traceability Matrix

This matrix is intentionally small at M0. It grows with implementation rather than pretending planned modules already exist.

| Requirement | Decision / design | Implementation | Verification | Evidence state |
|---|---|---|---|---|
| FR-001 Source identity | Architecture + Data/Protocol | `echora-core::Provenance`; protocol header partial | unit tests planned/partial | SOFTWARE_VERIFIED partial |
| FR-002 Source-mode separation | AGENTS, Data/Protocol | `SourceMode`; validation preventing simulated-as-measured | core unit test | SOFTWARE_VERIFIED |
| FR-003 Raw capture | Architecture/Sensing pipeline | not implemented | none | PROPOSED |
| FR-004 Replay | Architecture | not implemented | replay parity planned | PROPOSED |
| FR-005 Calibration | Architecture | types not yet implemented | state-machine tests planned | DESIGNED |
| FR-006 Quality/abstention | Architecture/Data | `SensingResult`, `AbstentionReason` | core unit tests | SOFTWARE_VERIFIED partial |
| FR-007 Presence/motion | Roadmap M2 | not implemented | controlled experiment planned | PROPOSED |
| FR-008 Respiration | Roadmap M2 | not implemented | reference experiment planned | PROPOSED |
| FR-009 Localization | Roadmap M3 | not implemented | grid/held-out tests planned | PROPOSED |
| FR-010 Pose | Roadmap M5 | not implemented | held-out reference evaluation planned | PROPOSED |
| FR-011 Reference capture | Experimentation | not implemented | sync/calibration evidence planned | PROPOSED |
| FR-012 API | Architecture | not implemented | contract tests planned | PROPOSED |
| FR-013 Visualization | Web contract | not implemented | UI invariants planned | DESIGNED |
| FR-014 Experiment records | Experimentation | template exists | review/manual until tooling | DESIGNED |

## Rule

Do not promote the evidence state based on documentation alone. Update this matrix only when the linked implementation and verification actually exist for the cited revision.
