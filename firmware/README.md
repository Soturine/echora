# Echora Firmware

## Status

**Designed, not yet implemented.**

Initial target: ESP32-S3 using ESP-IDF. ESP32-C6 is a secondary target after the S3 acquisition path is hardware-verified.

## Responsibilities

Firmware should do as little semantic inference as necessary during early milestones:

```text
Wi-Fi init
→ CSI callback
→ bounded capture buffer
→ metadata + sequence/timestamps
→ health
→ versioned transport
```

The host remains the authority for most DSP/inference while algorithms are still evolving.

## Proposed structure

```text
firmware/esp32-csi-node/
├── CMakeLists.txt
├── sdkconfig.defaults
├── components/
└── main/
    ├── app_main.c
    ├── csi_capture.c/.h
    ├── frame_protocol.c/.h
    ├── stream_sender.c/.h
    ├── node_config.c/.h
    └── health.c/.h
```

## Concurrency direction

On dual-core S3:

```text
Wi-Fi/CSI callback
    │ minimal copy
    ▼
bounded SPSC/ring buffer
    │
    ▼
stream task
    │
    ▼
UDP
```

Avoid blocking, heap-heavy or spectral processing inside the CSI callback.

## Configuration

Expected fields:
- SSID / provisioning state;
- target host/IP/port;
- node ID;
- capture profile;
- channel/bandwidth constraints;
- protocol version;
- log level.

Secrets must not be committed.

## Health

Expose:
- uptime/boot ID;
- frames captured;
- frames dropped;
- send failures;
- queue high-water mark;
- current RSSI/channel;
- firmware version;
- reset reason.

## HIL

Firmware is not considered working because it compiles. M1 requires hardware evidence on supported boards.
