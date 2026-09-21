# Echora Sensing Pipeline

## Principle

The pipeline preserves the distinction between raw radio observation and semantic human inference.

```text
RF environment
→ CSI acquisition
→ packet validation
→ timing / topology context
→ signal conditioning
→ features
→ estimator
→ fusion / tracking
→ bounded semantic state
→ UI / integration
```

No downstream layer may rewrite an upstream source as more authoritative than it is.

## Acquisition

Each CSI frame should retain, where available:
- node ID;
- radio/chip target;
- firmware version;
- protocol version;
- sequence number;
- device timestamp;
- host arrival timestamp;
- channel/bandwidth;
- RSSI/noise metadata;
- subcarrier count;
- complex CSI samples;
- capture flags;
- CRC/integrity state.

Raw data is the research evidence. Derived features are conveniences, not replacements for raw source lineage.

## Timing

At minimum track:
- device acquisition time;
- host receive time;
- aligned time;
- timing epoch/reboot;
- estimated offset/drift;
- uncertainty.

Multi-node localization/fusion is not enabled merely because several packets share a host timestamp.

## Conditioning

Candidate deterministic stages:
1. frame validity and plausibility checks;
2. subcarrier mask/null removal;
3. amplitude and phase extraction;
4. phase unwrap/sanitization where justified;
5. outlier handling (e.g. Hampel/robust statistics);
6. detrending/static-path suppression;
7. resampling with explicit gap policy;
8. subcarrier/link quality selection;
9. normalization bound to calibration identity.

Every irreversible transform should either retain raw input or be reproducible from a versioned capture.

## Feature families

Potential feature families:
- amplitude variance;
- phase difference / phase velocity;
- spectral power;
- Doppler-like temporal signatures;
- PCA/SVD components;
- correlation/coherence;
- motion energy;
- band-limited periodicity;
- learned embeddings.

No feature family is automatically “better.” Selection requires ablation and held-out evidence.

## Presence

Presence should combine:
- baseline-relative channel change;
- temporal consistency;
- link quality;
- optional learned classifier.

Failure cases include:
- moved furniture;
- AP/channel changes;
- moving fans;
- pets;
- neighboring activity;
- low SNR;
- calibration drift.

Output shape should include confidence/quality and abstention.

## Motion

Motion is initially coarse:
- still;
- motion detected;
- motion intensity/energy.

Activity labels such as walking/sitting/falling are separate learned/validated capabilities and shall not inherit motion evidence automatically.

## Respiration

Research pipeline:
- choose stable subcarrier/link observations;
- isolate low-frequency periodic motion;
- estimate spectral/temporal peak;
- reject physically implausible/noisy peaks;
- compare against independent reference;
- return uncertainty and quality.

Respiration is invalid when:
- presence is absent;
- window is too short;
- data gaps exceed tolerance;
- SNR/coherence is insufficient;
- motion contaminates the band beyond configured threshold.

## Heart-rate research

Cardiac micro-motion is a distinct harder problem. Do not derive “heart rate” merely because an FFT peak exists in a plausible frequency band.

Requirements before feature promotion:
- independent ECG/PPG reference;
- controlled and unconstrained trials;
- spectral-confound analysis;
- multiple subjects/positions;
- error distribution;
- rejection behavior;
- clear non-medical scope unless independently certified.

## Multi-node localization

Localization needs:
- known node/link geometry;
- coordinate frame;
- synchronized/aligned observations;
- per-link confidence;
- room calibration;
- solver uncertainty.

Return a point/region plus covariance/uncertainty, not a visually precise coordinate without support.

## Pose

Reference/training path:

```text
CSI windows ───────────────┐
                          ├→ aligned training sample
Kinect/RGB-D pose ─────────┘
```

Required before camera-free pose claim:
- synchronized pairing;
- calibration between frames;
- subject/room/session separation;
- metric definition locked;
- baseline comparison;
- confidence calibration;
- abstention;
- held-out evaluation.

## Output gating

Example conceptual rule:

```text
if source != LIVE:
    display source badge
if calibration invalid:
    withhold calibrated capabilities
if quality below minimum:
    abstain
if capability evidence level < visualization requirement:
    use lower-fidelity representation
```

Visualization fidelity must never exceed sensing evidence fidelity.
