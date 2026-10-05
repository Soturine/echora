# IEEE 802.11bf Compatibility Strategy

## Scope

IEEE 802.11bf defines WLAN sensing capabilities at the 802.11 MAC/PHY level. Echora does not currently claim standards compliance.

The architecture should nevertheless avoid coupling its core domain model to one vendor-specific CSI callback.

## Architectural rule

```text
vendor / standard sensing source
        ↓
source adapter
        ↓
Echora RF Observation Model
        ↓
DSP / calibration / inference / fusion
```

Examples of future adapters:
- ESP-IDF CSI;
- 802.11bf-capable WLAN sensing hardware;
- research NIC CSI;
- CIR/ranging source;
- BLE Channel Sounding;
- UWB;
- mmWave.

## Common observation semantics

The common model should preserve:
- source type;
- hardware identity;
- channel/frequency/bandwidth;
- antenna/link identity;
- timestamps;
- complex/channel measurement representation;
- measurement uncertainty/quality;
- vendor-specific extension metadata.

Do not force all future sources into an ESP32-specific subcarrier layout.

## Capability discovery

An adapter advertises:
- supported measurement types;
- frequency bands;
- antenna/link characteristics;
- timing capabilities;
- raw/processed availability;
- synchronization capabilities;
- security/authentication capabilities.

Capability Negotiation then determines what Echora can expose.

## Compliance language

Allowed:
- "architecture prepared for future 802.11bf adapters";
- "802.11bf-informed observation abstraction."

Not allowed without verification:
- "802.11bf compliant";
- "implements IEEE WLAN sensing";
- "ESP32 path is 802.11bf."

## Revisit trigger

Create a dedicated ADR when the first real 802.11bf-capable hardware backend is evaluated.
