# TRIBUNAL

**Justice 2.0 — improve caseflow without optimizing away rights.**

TRIBUNAL compares aggregate caseflow segments, calculates
resolution and pending-change measures, and separately tests counsel, victim
notice, pretrial liberty, and disparity floors. A throughput improvement cannot
pass if any rights floor fails.

The repo-local [VERDICT capability assessment](docs/vtrace/CAPABILITY_ASSESSMENT.md)
scores the current program **14/21**. Enactment, matched rights outcomes,
institutional delivery effects, and fiscal authority remain held until the
necessary observations exist.

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
cargo run --quiet -- candidate-baseline fixtures/official/cbo-hr1702-judges-act-2025-cost-estimate.tsv
cargo run --quiet -- candidate-held-pack fixtures/official/cbo-hr1702-judges-act-2025-cost-estimate.tsv
cargo run --quiet -- level2-baseline fixtures/official/aousc-texas-southern-2026-rights-baseline.tsv
cargo run --quiet -- level2-held-pack fixtures/official/aousc-texas-southern-2026-rights-baseline.tsv
cargo run --quiet -- held-pack fixtures/cedar-caseflow.tsv
cargo run --quiet -- official-baseline fixtures/official/aousc-fjcs-2025-caseflow.tsv
cargo run --quiet -- official-held-pack fixtures/official/aousc-fjcs-2025-caseflow.tsv
```

The Cedar fixture remains synthetic. The official baseline is a compact,
source-labelled derivation of AOUSC's published national toplines; its cases and
criminal defendants remain distinct source units and do not describe people,
rights quality, or the reason for any adjustment.

## Federal court-capacity candidate

TRIBUNAL now replays CBO's official estimate for H.R. 1702, the JUDGES Act of
2025. The committee-ordered proposal would add **65 permanent federal district
judgeships and one temporary judgeship**, phased every two years through 2035.

| FY2025-FY2035 CBO component | Outlays |
|---|---:|
| Judge compensation (direct spending) | $111M |
| Court operations and reports (subject to appropriation) | $283M |
| Combined | $394M |

FY2025 direct and appropriated outlays are each reported as less than $0.5M,
so the replay assigns neither a point value. The ten fully scored FY2026-FY2035
rows sum exactly to CBO's eleven-year totals; combined annual cost rises from
$8M in FY2026 to $71M in FY2035.

This is real capacity and cost evidence, not proof of faster or fairer justice.
CBO does not score filing-to-disposition time, aged pending cases, counsel,
pretrial liberty, victim notice, evidence quality, disparity, durable
resolution, or savings. The bill is not enacted, operating costs require
appropriation, and appointment, confirmation, staffing, facilities, security,
and technology milestones remain unresolved. The candidate therefore stays
held.

### Level 2 comparison baseline

The March 2026 AOUSC profile now anchors one district and one published case
type. In the Southern District of Texas, annual filings increased from 15,416
to **21,226** (+37.7%), pending work increased by **3,213** to 18,155, and
weighted filings reached **801 per authorized judgeship**. The profile reports
**46.0 vacant judgeship months** across 19 authorized seats.

Immigration accounted for **8,049 of 9,754 criminal felony-defendant filings**
(82.5%). That is a nested workload count, not a count of unique people and not
evidence that immigration cases caused the district-wide change.

This closes the district/case-type comparison-baseline gate, but not Level 2
admission. The source does not publish matched measures for counsel access,
pretrial detention, victim notice, disparity, or candidate effects. H.R. 1702
also remains unenacted. Those values stay null, so the candidate stays held.

## What this proves

- Filed, terminated, and pending work can be reconciled.
- Published aggregate series may need an explicit adjustment rather than a
  manufactured accounting identity.
- Counsel, liberty, victim protection, and disparity remain independent floors.
- More terminations do not automatically prove just or durable resolution.
- More judgeships do not automatically prove faster, safer, or fairer outcomes.
- Taxlane can receive a held JUS finding without acquiring judicial authority.

## Validate

```powershell
cargo fmt --check
cargo test --workspace --all-targets
cargo run --quiet -- analyze fixtures/cedar-caseflow.tsv
cargo run --quiet -- official-baseline fixtures/official/aousc-fjcs-2025-caseflow.tsv
cargo run --quiet -- level2-baseline fixtures/official/aousc-texas-southern-2026-rights-baseline.tsv
```

Official anchor: [Federal Judicial Caseload Statistics 2025](https://www.uscourts.gov/data-news/reports/statistical-reports/federal-judicial-caseload-statistics/federal-judicial-caseload-statistics-2025).

Candidate sources: [CBO H.R. 1702 cost estimate](https://www.cbo.gov/publication/61263),
[Congress.gov H.R. 1702 status](https://www.congress.gov/bill/119th-congress/house-bill/1702/all-info),
and [Judicial Conference 2025 judgeship recommendations](https://www.uscourts.gov/data-news/judiciary-news/2025/03/11/judiciary-seeks-71-judgeships-meet-growing-caseloads).

Level 2 source: [AOUSC Federal Court Management Statistics, March 31, 2026 district profiles](https://www.uscourts.gov/sites/default/files/document/fcms_na_distprofile0331.2026.pdf).

## Boundary

TRIBUNAL is aggregate research software. It is not legal advice, an individual
risk assessment, detention or release guidance, a judicial recommendation,
official score, savings claim, allocation, rate instruction, or release authorization.
