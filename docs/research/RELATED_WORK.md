# Related Work and Engineering Lessons

This file records architectural/research inputs. It is not an endorsement of every claim made by referenced projects.

## RuView / WiFi-DensePose

Reference: https://github.com/ruvnet/RuView

Useful patterns observed:
- real ESP32 CSI capture path;
- explicit live/replay/simulated source handling after earlier fallback problems;
- firmware + host processing separation;
- raw/derived sensing pipeline;
- calibration lifecycle;
- deterministic proof/replay concepts;
- ADR-heavy architectural traceability;
- hardware validation notes;
- public corrections/retractions of unsupported claims;
- CI for firmware/software/model boundaries.

Lessons adopted by Echora:
1. simulator data must never silently appear as live;
2. pose visualization can greatly exceed actual sensing evidence;
3. one favorable metric can be invalid because of split/metric/label leakage;
4. raw CSI and provenance should survive long enough to re-evaluate algorithms;
5. negative results and retractions are useful engineering evidence;
6. a single ESP32 is primarily an acquisition/presence/motion research node, not proof of camera-grade pose;
7. physiological claims require independent references.

Echora does **not** copy RuView's capability claims or architecture wholesale. Each capability must earn its own evidence.

## Espressif ESP-CSI

Reference: https://github.com/espressif/esp-csi

Useful as an upstream technical reference for ESP32 CSI acquisition and examples. Device/IDF support must be verified against the version used by Echora rather than assumed from historical examples.

## Wi-Fi human-sensing research

Relevant research families include:
- human activity recognition from CSI;
- device-free localization;
- respiration/micro-motion sensing;
- person counting;
- 2D/3D pose estimation from Wi-Fi;
- multimodal teacher/reference capture.

Research numbers are only comparable when hardware, bandwidth, antenna count, topology, dataset split and metric definition are comparable.

## Reference-project intake rule

For any external project/paper:

```text
claim
→ source
→ hardware/data/metric
→ reproducibility
→ applicability to Echora
→ decision
```

A GitHub README or demo video is evidence of a claim, not proof that the capability transfers to Echora.
