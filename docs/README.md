# Echora Documentation

This directory is the maintained engineering portal for Echora.

## Authority map

- **Normative/project intent:** [product/PRD.md](product/PRD.md)
- **Approved architectural direction:** [architecture/ARCHITECTURE.md](architecture/ARCHITECTURE.md) and ADRs
- **Technical design:** engineering documents
- **Verification/assurance:** quality documents and test evidence
- **Operational targets:** operations documents
- **Historical changes:** repository changelog and superseded ADRs

If code and documentation disagree, do not silently edit the requirement to match code. Record the divergence and resolve it explicitly.

## Product
- [PRD](product/PRD.md)

## Architecture
- [System architecture](architecture/ARCHITECTURE.md)
- [ADR 0001 — architecture baseline](architecture/adr/0001-architecture-baseline.md)

## Engineering
- [Stack](engineering/STACK.md)
- [Sensing pipeline](engineering/SENSING_PIPELINE.md)
- [Data and protocol contracts](engineering/DATA_AND_PROTOCOL.md)
- [Experimentation](engineering/EXPERIMENTATION.md)
- [Development workflow](engineering/DEVELOPMENT.md)

## Quality
- [Test strategy](quality/TEST_STRATEGY.md)
- [QA and assurance](quality/QA_ASSURANCE.md)

## Security
- [Security and privacy](security/SECURITY_PRIVACY.md)

## Operations
- [SLO/SLA](operations/SLO_SLA.md)

## Project-wide invariant

```text
measurement quality
+ provenance
+ calibration
+ uncertainty
+ evidence scope
> visual impressiveness
```
