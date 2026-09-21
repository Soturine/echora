# Echora Development Workflow

## Work model

Use small, reviewable, evidence-bearing increments.

```text
need
→ requirement / issue
→ design or ADR if needed
→ implementation
→ verification
→ physical/experimental validation when applicable
→ documentation/evidence
→ release
```

## Branch and commit guidance

- one coherent concern per change unit;
- conventional-style commit messages are preferred;
- do not mix broad formatting/refactors with sensing behavior changes;
- changes to protocol/evidence/calibration/model semantics require explicit migration/compatibility consideration.

## Code quality

Rust:
- `cargo fmt --check`;
- clippy with warnings denied for maintained crates where practical;
- unit/property/integration tests;
- no unchecked panics on untrusted boundary input.

Firmware:
- warning-clean supported build;
- bounded buffers;
- explicit task ownership;
- watchdog/restart behavior documented;
- hardware target matrix.

Python:
- ruff;
- pytest;
- typing on stable interfaces;
- deterministic seeds where relevant;
- no notebook-only production path.

Web:
- TypeScript strict mode;
- lint/typecheck;
- component/unit tests;
- Playwright for end-to-end operator flows once UI exists;
- accessibility checks.

## Configuration

Configuration precedence must be documented when implemented. Recommended order:

```text
compiled safe defaults
< config file
< environment
< explicit CLI
```

Secrets are not valid normal configuration values for committed files.

## Feature status

Do not merge a new feature with ambiguous maturity. Mark it in docs/API as:
- experimental;
- supported;
- deprecated;
- disabled/gated.

## Review checklist

For sensing changes:
- raw-vs-derived semantics preserved?
- source mode preserved?
- units/timestamps correct?
- calibration invalidation considered?
- quality/abstention behavior defined?
- replay still equivalent?
- experiment/metric impact considered?
- privacy/security impact considered?
- docs and tests aligned?

## Releases

A release should include:
- exact revision/tag;
- changelog;
- supported hardware/runtime matrix;
- migrations;
- known limitations;
- test evidence;
- artifact hashes/checksums where practical;
- model/calibration compatibility notes.

A green CI run on a different commit is not release evidence.

## AI-assisted changes

AI may generate code/docs/tests, but generated output is not evidence of correctness. Review, execute relevant tests and validate physical claims independently.
