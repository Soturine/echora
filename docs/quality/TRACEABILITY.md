# Echora Traceability Matrix

This matrix grows with implementation rather than pretending planned modules already exist.

| Requirement / concern | Decision / design | Implementation | Verification | Evidence state |
|---|---|---|---|---|
| FR-001 Source identity | Architecture + Data/Protocol | `echora-core::Provenance`; protocol header partial | unit tests planned/partial | SOFTWARE_VERIFIED partial |
| FR-002 Source-mode separation | AGENTS, Data/Protocol | `SourceMode`; validation preventing simulated-as-measured | core unit test | SOFTWARE_VERIFIED |
| FR-003 Raw capture | Architecture/Sensing/CSI hardware matrix | not implemented | replay/capture acceptance planned | PROPOSED |
| FR-004 Replay | Architecture | not implemented | replay parity planned | PROPOSED |
| FR-005 Calibration | Calibration/Drift | schema exists; runtime not implemented | state-machine + invalidation tests planned | DESIGNED |
| FR-006 Quality/abstention | Architecture/Data | `SensingResult`, `AbstentionReason` | core unit tests | SOFTWARE_VERIFIED partial |
| FR-007 Presence/motion | Roadmap M2 | not implemented | controlled experiment planned | PROPOSED |
| FR-008 Respiration | Roadmap M2 | not implemented | reference experiment planned | PROPOSED |
| FR-009 Localization | RF geometry + roadmap M3 | spatial/topology schemas exist; solver absent | grid/held-out tests planned | DESIGNED |
| FR-010 Pose | Visualization + roadmap M5 | not implemented | held-out reference evaluation planned | PROPOSED |
| FR-011 Reference capture | Experimentation + time sync | not implemented | sync/calibration evidence planned | PROPOSED |
| FR-012 API | Architecture | not implemented | contract tests planned | PROPOSED |
| FR-013 Visualization | Visualization architecture + web contract | not implemented | UI invariants planned | DESIGNED |
| FR-014 Experiment records | Experimentation | template exists | review/manual until tooling | DESIGNED |
| RF topology identity | Hardware matrix + topology schema | schema exists | schema validation planned | DESIGNED |
| Coordinate/time semantics | Coordinate frames/time sync | not implemented | alignment fixtures planned | DESIGNED |
| Calibration drift | Calibration/Drift | not implemented | invalidation/drift scenarios planned | DESIGNED |
| Capability negotiation | Capability Negotiation + schema | schema exists; engine absent | rule/property tests planned | DESIGNED |
| Renderer ceiling | Visualization architecture | not implemented | UI contract/E2E planned | DESIGNED |
| Dataset leakage control | Dataset/Benchmark protocol | process defined | benchmark tooling planned | DESIGNED |
| Claim governance | Claim/Evidence Matrix | authoritative doc exists | release review | DESIGNED |
| Future WLAN sensing | 802.11bf strategy | adapter boundary designed | no compliance verification | DESIGNED |

## Rule

Do not promote the evidence state based on documentation alone. Update this matrix only when the linked implementation and verification actually exist for the cited revision.
