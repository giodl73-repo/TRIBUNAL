# TRIBUNAL

**Justice 2.0 — improve caseflow without optimizing away rights.**

TRIBUNAL's first feature compares aggregate caseflow segments, calculates
resolution and pending-change measures, and separately tests counsel, victim
notice, pretrial liberty, and disparity floors. A throughput improvement cannot
pass if any rights floor fails.

In the fictional Cedar example, 12,000 matters enter and 10,800 terminate, but
the counsel-access floor is 84%, victim-notice floor is 79%, and the longest
median pretrial detention is 41 days. The result is therefore held despite
strong aggregate termination volume.

## Try it

```powershell
cargo run --quiet -- analyze fixtures/cedar-caseflow.tsv
cargo run --quiet -- held-pack fixtures/cedar-caseflow.tsv
```

The source spine uses the Administrative Office of the U.S. Courts' Federal
Judicial Caseload Statistics 2025 as an aggregate workload anchor. The fixture
is synthetic and does not describe any court, person, or case.

## What this proves

- Filed, terminated, and pending work can be reconciled.
- Counsel, liberty, victim protection, and disparity remain independent floors.
- More terminations do not automatically prove just or durable resolution.
- Taxlane can receive a held JUS finding without acquiring judicial authority.

## Validate

```powershell
cargo fmt --check
cargo test --all-targets
cargo run --quiet -- analyze fixtures/cedar-caseflow.tsv
```

Official anchor: [Federal Judicial Caseload Statistics 2025](https://www.uscourts.gov/data-news/reports/statistical-reports/federal-judicial-caseload-statistics/federal-judicial-caseload-statistics-2025).

## Boundary

TRIBUNAL is aggregate research software. It is not legal advice, an individual
risk assessment, detention or release guidance, a judicial recommendation,
official score, savings claim, allocation, rate instruction, or release authorization.
