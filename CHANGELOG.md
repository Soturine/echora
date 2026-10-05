# Changelog

All notable changes to Echora will be documented here.

The project follows Semantic Versioning once public versioned artifacts begin shipping.

## [Unreleased]

### Added
- Initial project charter.
- Engineering contract.
- Evidence-gated roadmap.
- Product, architecture, sensing, QA, security and operations documentation.
- Initial Rust core domain scaffold and CI.
- Wi-Fi / RF sensing ecosystem survey covering RuView, Espressif and research references.
- CSI acquisition and hardware target matrix with raw and edge firmware profiles.
- RF geometry/Fresnel spatial model.
- Explicit coordinate-frame and time-synchronization strategy.
- Calibration lifecycle and drift/invalidation model.
- Topology-aware capability negotiation and renderer ceilings.
- Evidence-constrained 3D visualization architecture.
- Dataset and benchmark protocol with leakage-resistant splits.
- Claim/evidence matrix.
- IEEE 802.11bf compatibility strategy.
- Schemas for topology, calibration, spatial state and capability status.
- Model/data/capability/hardware/calibration documentation templates.

### Changed
- Roadmap expanded to include hardware characterization, spatial likelihood fields, capability negotiation, benchmark governance and future RF-dense representations.
- Architecture now treats ESP32 CSI as one adapter behind a hardware-neutral RF observation boundary.
- Web design now separates RF Lab, Signal, Occupancy, Track, Pose and Reference views.
