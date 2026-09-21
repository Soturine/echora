# Echora Security and Privacy

## Security stance

Echora processes network packets, firmware, model artifacts and potentially sensitive human-sensing data. Security is part of the architecture, not a final scanner step.

## Assets

Protect:
- raw CSI captures;
- derived human-state data;
- calibration/topology;
- model artifacts;
- firmware/configuration;
- API credentials;
- experiment metadata;
- release artifacts.

## Trust boundaries

1. sensor node → host network;
2. capture/replay import;
3. model import;
4. reference-sensor import;
5. CLI/configuration input;
6. API/browser boundary;
7. optional MQTT/external integration.

## Baseline controls

- validate all untrusted lengths/ranges/enums;
- explicit maximum packet/capture sizes;
- never deserialize arbitrary executable objects;
- secrets via environment/secret store, not repository;
- localhost binding by default for early host services;
- authentication required before remote exposure;
- least privilege for capture/model directories;
- dependency/advisory scanning;
- artifact checksums; signatures for releases when mature;
- logs avoid raw credentials and unnecessary human-sensitive payloads.

## Network

Initial UDP sensing transport prioritizes controlled local experiments, not hostile WAN exposure.

Before broader deployment:
- authenticated node identity;
- replay protection/session identity;
- integrity/authenticity beyond CRC;
- network segmentation;
- rate limiting/resource bounds;
- secure provisioning.

CRC detects corruption; it is not authentication.

## Firmware

- credentials must not be hard-coded in source;
- provisioning flow should minimize credential exposure;
- safe reboot/watchdog behavior;
- OTA, if introduced, must verify trusted artifacts and support rollback;
- debug interfaces and verbose dumps reviewed for production profiles.

## Models and supply chain

Treat model files as untrusted data:
- bounded parser;
- format allowlist;
- checksum/version;
- expected feature contract;
- no pickle-style arbitrary code loading in trusted runtime;
- document training provenance and licenses.

Third-party dependencies/models/datasets require license and provenance review.

## Privacy

Camera-free does not mean privacy-free.

CSI-derived data may reveal:
- occupancy;
- movement;
- routines;
- location;
- sleep/activity patterns;
- physiological signals;
- potentially identity-correlated patterns.

Therefore:
- process locally by default;
- minimize raw-data retention;
- purpose-bind captures;
- use pseudonymous subject IDs;
- do not publish raw human captures by default;
- document consent/authority for controlled collection;
- define retention/deletion policy before field studies;
- separate research/reference video from production sensing.

## Derived-data principle

Deleting video does not automatically remove privacy sensitivity if pose, location or physiological features remain.

## Abuse considerations

Potential misuse includes covert monitoring, stalking, unauthorized occupancy inference and sensitive physiological surveillance. Productization for such uses is out of scope. Public documentation should avoid implying invisible surveillance as a benefit.

## Incident readiness

When external users/deployments exist, add:
- SECURITY.md reporting policy;
- vulnerability triage;
- credential rotation;
- release/advisory process;
- capture exposure response;
- model/data provenance incident handling.

## Security verification

CI/release gates should evolve toward:
- secret scanning;
- dependency auditing;
- SBOM;
- static analysis;
- fuzzing parsers;
- artifact provenance;
- signed release checksums.
