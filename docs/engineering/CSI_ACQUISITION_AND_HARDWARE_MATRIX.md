# CSI Acquisition and Hardware Matrix

## Purpose

Echora must not encode one ESP32 board as the definition of Wi-Fi sensing. Hardware targets are acquisition adapters with different RF capabilities, CPU/memory budgets and CSI semantics.

## Initial hardware strategy

| Target | Role | Initial priority | Notes |
|---|---|---:|---|
| ESP32-S3 | baseline development / raw CSI node | P0 | strong ecosystem, dual-core host for capture/transport work |
| ESP32-C6 | Wi-Fi 6 / HE research node | P1 | evaluate CSI differences and low-power scenarios |
| ESP32-C5 | future RF research target | P1/P2 | candidate for newer dual-band / sensing experiments when tooling is stable |
| ESP32-C3 / classic ESP32 | compatibility / acquisition-lite | P2 | useful only where actual CSI APIs and throughput meet requirements |
| research NIC / PicoScenes-class source | advanced comparison | future | richer MIMO/CSI can be used as a reference backend |
| mmWave / UWB / BLE CS | non-Wi-Fi adapters | future | map through common observation contracts |

Priority labels are roadmap intent, not support claims.

## Two firmware profiles

### `echora-node-raw`

Research-first node.

Responsibilities:
- acquire raw CSI;
- preserve chip/radio metadata;
- sequence/timestamp frames;
- bounded buffering;
- node health;
- versioned transport;
- minimal irreversible DSP.

Use when:
- collecting datasets;
- changing algorithms;
- validating phase/amplitude handling;
- performing replayable research.

### `echora-node-edge`

Operational sensing node.

Responsibilities may include:
- acquisition;
- calibration state;
- supported edge motion/presence features;
- local quality metrics;
- MQTT/home automation integration;
- OTA/provisioning.

Rule: an edge capability must be validated against the raw/reference path before promotion.

## Acquisition topology vocabulary

Every run records topology explicitly:

```text
AP / transmitter
receiver node(s)
channel / center frequency
bandwidth
link direction
antenna orientation
node XYZ + orientation
traffic generation mode
packet rate target
actual CSI frame rate
environment ID
```

Do not infer geometry from node naming conventions.

## RF configuration dimensions

Track, where exposed:
- 2.4 / 5 / future supported band;
- channel;
- channel width;
- PHY mode;
- MCS/rate constraints where relevant;
- LTF / HE-LTF details where relevant;
- antenna selection;
- RSSI/noise;
- number and interpretation of CSI coefficients;
- chip/IDF-specific CSI flags;
- AP/router model and firmware.

## Acquisition quality

A capture is not accepted as "healthy" solely because packets arrive.

Measure:
- actual CSI frames/s;
- sequence gaps;
- host UDP loss;
- device ring-buffer drops;
- out-of-order/duplicates;
- timestamp jitter;
- reboot/timing epochs;
- RSSI/SNR distribution;
- coefficient validity/missingness;
- channel changes;
- thermal/power/restart behavior.

## External antenna research

For targets that support external antennas, record:
- antenna model;
- gain/pattern where known;
- connector/cable;
- polarization;
- orientation;
- placement uncertainty.

External antennas may improve repeatability, but they also create a new configuration dimension that invalidates earlier calibration unless explicitly shown otherwise.

## Support levels

Use:

```text
DISCOVERED
BUILDS
STREAMS
HIL_VERIFIED
CHARACTERIZED
SUPPORTED
```

Example: a chip compiling an ESP-IDF example is only `BUILDS`, not `SUPPORTED`.

## M1 acceptance matrix

For the first supported raw node:
- clean build from documented toolchain;
- provisioning reproducible;
- live CSI stream identified as LIVE;
- sequence/loss metrics visible;
- raw capture persisted;
- replay parity demonstrated;
- reconnect/reboot handled;
- malformed protocol input cannot crash host;
- exact board/IDF configuration recorded.

## Compatibility rule

Device-specific adapters translate into the common Echora observation model while retaining vendor metadata. Never discard information solely to make two hardware families look identical.
