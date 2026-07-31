# Pulse 02 — official federal caseflow baseline

Added a replayable AOUSC 2025 national caseflow baseline across appeals,
district civil, district criminal defendants, and bankruptcy. The result keeps
the source units distinct and reports the implied adjustment instead of forcing
an exact identity. It does not infer rights quality, causes, capacity, costs,
savings, or a candidate.

Validation:

```powershell
cargo fmt --check
cargo test --all-targets
cargo run --quiet -- official-baseline fixtures/official/aousc-fjcs-2025-caseflow.tsv
cargo run --quiet -- official-held-pack fixtures/official/aousc-fjcs-2025-caseflow.tsv
```
