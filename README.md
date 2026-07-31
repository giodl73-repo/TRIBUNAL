# TRIBUNAL

**Justice 2.0 — improve caseflow without optimizing away rights.**

TRIBUNAL compares aggregate caseflow segments, calculates
resolution and pending-change measures, and separately tests counsel, victim
notice, pretrial liberty, and disparity floors. A throughput improvement cannot
pass if any rights floor fails.

Its first official run now reconciles four AOUSC national workload series. The
published 2024 start, 2025 filings, terminations, and 2025 end counts imply a
net **+554 statistical adjustment**; district civil has the largest absolute
adjustment at **579 cases**. TRIBUNAL preserves that adjustment instead of
silently forcing the source toplines into an exact identity.

In the fictional Cedar example, 12,000 matters enter and 10,800 terminate, but
the counsel-access floor is 84%, victim-notice floor is 79%, and the longest
median pretrial detention is 41 days. The result is therefore held despite
strong aggregate termination volume.

## Try it

```powershell
cargo run --quiet -- analyze fixtures/cedar-caseflow.tsv
cargo run --quiet -- held-pack fixtures/cedar-caseflow.tsv
cargo run --quiet -- official-baseline fixtures/official/aousc-fjcs-2025-caseflow.tsv
cargo run --quiet -- official-held-pack fixtures/official/aousc-fjcs-2025-caseflow.tsv
```

The Cedar fixture remains synthetic. The official baseline is a compact,
source-labelled derivation of AOUSC's published national toplines; its cases and
criminal defendants remain distinct source units and do not describe people,
rights quality, or the reason for any adjustment.

## What this proves

- Filed, terminated, and pending work can be reconciled.
- Published aggregate series may need an explicit adjustment rather than a
  manufactured accounting identity.
- Counsel, liberty, victim protection, and disparity remain independent floors.
- More terminations do not automatically prove just or durable resolution.
- Taxlane can receive a held JUS finding without acquiring judicial authority.

## Validate

```powershell
cargo fmt --check
cargo test --all-targets
cargo run --quiet -- analyze fixtures/cedar-caseflow.tsv
cargo run --quiet -- official-baseline fixtures/official/aousc-fjcs-2025-caseflow.tsv
```

Official anchor: [Federal Judicial Caseload Statistics 2025](https://www.uscourts.gov/data-news/reports/statistical-reports/federal-judicial-caseload-statistics/federal-judicial-caseload-statistics-2025).

## Boundary

TRIBUNAL is aggregate research software. It is not legal advice, an individual
risk assessment, detention or release guidance, a judicial recommendation,
official score, savings claim, allocation, rate instruction, or release authorization.
