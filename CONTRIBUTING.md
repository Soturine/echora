# Contributing to Echora

Echora welcomes engineering and research contributions that preserve evidence integrity.

## Before coding

Read:
- [README.md](README.md)
- [AGENTS.md](AGENTS.md)
- [docs/README.md](docs/README.md)
- relevant architecture/engineering/QA document.

## Contribution principles

- solve a defined problem;
- keep changes coherent and reviewable;
- add tests/evidence with behavior changes;
- distinguish proposed vs implemented vs validated;
- do not upgrade capability claims without evidence;
- document protocol/architecture changes in ADRs;
- preserve privacy and dataset provenance.

## Pull request content

A meaningful PR should explain:
- problem;
- approach;
- affected requirement/ADR;
- tests executed;
- hardware/experiment evidence if applicable;
- compatibility/migration;
- security/privacy implications;
- limitations and remaining gaps.

## Commit style

Conventional-style prefixes are encouraged:

```text
feat:
fix:
docs:
test:
refactor:
perf:
build:
ci:
chore:
```

## Experimental results

Do not paste only the best run. Include configuration, split, baseline, failures and limitations. Results derived solely from synthetic data must say so.

## Data

Do not commit:
- private raw captures;
- reference videos of people;
- credentials;
- large model artifacts without an approved artifact strategy;
- datasets with unclear rights/consent.

## Code of evidence

The strongest rule in this repository:

> Never make the repository look more complete than the evidence says it is.
