# Echora Risk Register

This is a living engineering risk register. It does not use arbitrary numeric scoring; priority is based on impact, likelihood/context and decision urgency.

| ID | Risk | Current treatment | Status |
|---|---|---|---|
| R-001 | Demo visualization overstates sensing fidelity | evidence-gated UI; no skeleton without pose evidence | Open |
| R-002 | Simulator/replay mistaken for live data | typed source mode; fail-closed; UI badge; regression tests | Mitigated by design |
| R-003 | Model learns room/session shortcuts instead of humans | cross-room/person/session/device splits; leakage checks | Open |
| R-004 | Calibration drift from moved nodes/furniture/channel | calibration identity + invalidation + drift monitoring | Open |
| R-005 | Single-node topology insufficient for spatial/physiological claims | staged roadmap; multi-link validation | Open |
| R-006 | Heart-rate spectral peaks are artifacts | independent reference, rejection gates, experimental-only label | Open |
| R-007 | Human sensing creates privacy harms despite no camera | local processing, minimization, consent/purpose/retention | Open |
| R-008 | UDP packet spoof/replay in non-lab deployment | local-only early scope; authenticated protocol before field use | Open |
| R-009 | Reference camera/keypoint model treated as perfect ground truth | record uncertainty/version/sync/visibility | Open |
| R-010 | Raw capture volume becomes unmanageable | measure throughput; configurable retention; derived-index separation | Open |
| R-011 | Rust/Python model parity drift | export parity tests and shared feature contract | Future |
| R-012 | Firmware timing/buffer pressure biases CSI | HIL loss/jitter/buffer telemetry | Open |
| R-013 | Published datasets expose sensitive routines/physiology | publication review and sanitization; no default raw upload | Open |
| R-014 | Third-party project claims copied without reproduction | related-work evidence grading; reproduce before adoption | Open |

## Closure rule

A risk is not closed because a mitigation is documented. Closure requires evidence that the relevant control exists and, when material, that it was tested.
