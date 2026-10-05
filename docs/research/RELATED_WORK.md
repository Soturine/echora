# Related Work and Engineering Lessons

This file summarizes lessons. The broader catalog is maintained in [ECOSYSTEM_SURVEY.md](ECOSYSTEM_SURVEY.md).

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
7. physiological claims require independent references;
8. visualization maturity and sensing maturity must remain separate;
9. point-cloud provenance must distinguish camera/depth reconstruction, multimodal fusion and RF-only inference.

Echora does **not** copy RuView's capability claims or architecture wholesale. Each capability must earn its own evidence.

## Espressif ESP-CSI

Reference: https://github.com/espressif/esp-csi

Useful as an upstream technical reference for ESP32 CSI acquisition, sensing demos and hardware-family experiments. Device/IDF support must be verified against the exact version used by Echora.

## Other reference families

The ecosystem survey additionally covers:
- ESP32-CSI-Tool;
- ESPectre;
- CSIKit;
- SenseFi / CSI benchmarks;
- Widar-style physics-aware representations;
- respiration/micro-motion research;
- Person-in-WiFi / DensePose / RF-Pose;
- MM-Fi;
- CSI-to-point-cloud research;
- IEEE 802.11bf WLAN sensing.

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
