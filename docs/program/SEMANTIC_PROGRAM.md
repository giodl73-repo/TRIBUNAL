# TRIBUNAL semantic program

## Product achievement

| Feature | Demonstrated question | Current result |
|---|---|---|
| Caseflow | Can throughput improve while rights fail? | Cedar remains held on counsel, victim, liberty floors |
| Official scale | Do national workload stocks and flows reconcile? | Four series with explicit statistical adjustment |
| Candidate envelope | What does H.R. 1702 add and cost? | 66 judgeships; $111M direct + $283M appropriated |
| Comparison baseline | What district workload precedes enactment? | 21,226 filings; 18,155 pending; rights null |
| Scenarios | What happens under workload stress and recovery? | 18,155 → 22,000 → 17,000 synthetic pending |
| Realization | Where does rights-bounded resolution lose matters? | 100,000 filed → 65,000 rights-compliant → 60,000 durable |
| Accounting | Are transition and security costs visible? | $454M synthetic total including $60M transition |
| Alternatives | Which paths clear pending and rights floors? | Two of three; none selected |
| Incidence | Who carries burden? | Five groups reconcile; accused people carry largest burden |
| Delivery | Can added capacity be delivered? | Five of ten gates; five institutional gates missing |
| Adaptation | Can workload trigger review without action? | Immutable successor version 2 |
| Comparison | Is a definition-matched interval gap visible? | 235 days illustrative; not official or a target |
| Held handoff | Can Taxlane inspect without judicial authority? | Complete held pack; zero admission authority |

## Validation

```powershell
cargo fmt --check
cargo test --workspace --all-targets
cargo run --quiet -- program-held-pack fixtures/synthetic/justice-semantic-program.tsv
```

## Remaining evidence work

Enactment, appropriations, appointments, staffing, facilities, security, and
matched counsel, liberty, victim, disparity, cost, and durable-resolution
evidence remain prerequisites for candidate review.
