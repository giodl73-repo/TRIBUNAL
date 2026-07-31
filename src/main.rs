use std::env;
use std::fs;
use std::process::ExitCode;

mod official;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Segment {
    name: String,
    filed: u64,
    terminated: u64,
    pending_start: u64,
    pending_end: u64,
    median_days: u64,
    counsel_bps: u64,
    victim_notice_bps: u64,
    detention_days: u64,
    disparity_milli: u64,
}

#[cfg(test)]
mod lane_pack_contract_tests {
    use super::*;

    #[test]
    fn held_pack_exposes_every_taxlane_contract_section() {
        let fixture = include_str!("../fixtures/cedar-caseflow.tsv");
        let pack = held_pack_json(&analyze(&parse(fixture).unwrap()));
        for section in [
            "identity",
            "scope",
            "source_custody",
            "problem",
            "intervention",
            "outcomes",
            "service_floors",
            "costs",
            "fiscal_bridge",
            "adaptive_pathways",
            "delivery",
            "overlap",
            "readiness",
            "claim_boundaries",
        ] {
            assert!(
                pack.contains(&format!("\"{section}\":")),
                "missing {section}"
            );
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Analysis {
    filed: u64,
    terminated: u64,
    pending_change: i64,
    resolution_bps: u64,
    longest_median_days: u64,
    counsel_floor_bps: u64,
    victim_notice_floor_bps: u64,
    longest_detention_days: u64,
    worst_disparity_milli: u64,
    accounting_reconciles: bool,
    rights_floor_pass: bool,
}

fn parse(input: &str) -> Result<Vec<Segment>, String> {
    let mut rows = Vec::new();
    for (index, line) in input.lines().enumerate() {
        let line_number = index + 1;
        if line.is_empty() || line.starts_with('#') || line.starts_with("segment\t") {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 10 {
            return Err(format!("line {line_number}: expected 10 fields"));
        }
        let number = |field: usize| {
            fields[field]
                .parse::<u64>()
                .map_err(|_| format!("line {line_number}: invalid integer"))
        };
        let row = Segment {
            name: fields[0].to_owned(),
            filed: number(1)?,
            terminated: number(2)?,
            pending_start: number(3)?,
            pending_end: number(4)?,
            median_days: number(5)?,
            counsel_bps: number(6)?,
            victim_notice_bps: number(7)?,
            detention_days: number(8)?,
            disparity_milli: number(9)?,
        };
        if row.counsel_bps > 10_000 || row.victim_notice_bps > 10_000 {
            return Err(format!("line {line_number}: basis points exceed 10000"));
        }
        rows.push(row);
    }
    if rows.is_empty() {
        return Err("at least one caseflow segment is required".into());
    }
    Ok(rows)
}

fn analyze(rows: &[Segment]) -> Analysis {
    let filed: u64 = rows.iter().map(|row| row.filed).sum();
    let terminated: u64 = rows.iter().map(|row| row.terminated).sum();
    let pending_start: u64 = rows.iter().map(|row| row.pending_start).sum();
    let pending_end: u64 = rows.iter().map(|row| row.pending_end).sum();
    let expected_end = pending_start as i64 + filed as i64 - terminated as i64;
    let counsel_floor = rows.iter().map(|row| row.counsel_bps).min().unwrap_or(0);
    let notice_floor = rows
        .iter()
        .map(|row| row.victim_notice_bps)
        .min()
        .unwrap_or(0);
    let detention = rows.iter().map(|row| row.detention_days).max().unwrap_or(0);
    let disparity = rows
        .iter()
        .map(|row| row.disparity_milli)
        .max()
        .unwrap_or(0);
    Analysis {
        filed,
        terminated,
        pending_change: pending_end as i64 - pending_start as i64,
        resolution_bps: if filed == 0 {
            0
        } else {
            terminated * 10_000 / filed
        },
        longest_median_days: rows.iter().map(|row| row.median_days).max().unwrap_or(0),
        counsel_floor_bps: counsel_floor,
        victim_notice_floor_bps: notice_floor,
        longest_detention_days: detention,
        worst_disparity_milli: disparity,
        accounting_reconciles: expected_end == pending_end as i64,
        rights_floor_pass: counsel_floor >= 9_000
            && notice_floor >= 9_000
            && detention <= 30
            && disparity <= 1_100,
    }
}

fn analysis_json(result: &Analysis) -> String {
    format!(
        "{{\"schema\":\"tribunal.rights-bounded-caseflow.v1\",\"filed\":{},\"terminated\":{},\"pending_change\":{},\"resolution_bps\":{},\"longest_median_days\":{},\"counsel_floor_bps\":{},\"victim_notice_floor_bps\":{},\"longest_detention_days\":{},\"worst_disparity_milli\":{},\"accounting_reconciles\":{},\"rights_floor_pass\":{},\"throughput_alone_is_justice\":false}}",
        result.filed, result.terminated, result.pending_change, result.resolution_bps,
        result.longest_median_days, result.counsel_floor_bps,
        result.victim_notice_floor_bps, result.longest_detention_days,
        result.worst_disparity_milli, result.accounting_reconciles, result.rights_floor_pass
    )
}

fn held_pack_json(result: &Analysis) -> String {
    format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"tribunal:cedar-rights-bounded-caseflow:v1\",\"track\":\"JUS\",\"domain_repository\":\"TRIBUNAL\",\"candidate_id\":\"cedar-rights-bounded-caseflow\",\"candidate_name\":\"Cedar rights-bounded caseflow screen\",\"fiscal_owner\":\"TAXLANE\"}},\"scope\":{{\"geography\":\"synthetic Cedar\",\"population_or_network\":\"aggregate caseflow segments\",\"ownership\":\"illustrative justice system\",\"time_basis\":\"annual illustration\",\"unit_basis\":\"matters days basis points and ratios\",\"included\":\"flow counsel notice detention disparity\",\"excluded\":\"individual cases and judicial decisions\"}},\"source_custody\":{{\"source_id\":\"AOUSC-FJCS-2025\",\"publisher\":\"Administrative Office of the United States Courts\",\"source_path_or_url\":\"https://www.uscourts.gov/data-news/reports/statistical-reports/federal-judicial-caseload-statistics/federal-judicial-caseload-statistics-2025\",\"vintage\":\"12 months ending 2025-03-31\",\"capture_status\":\"registry_linked\",\"checksum_or_null\":null}},\"problem\":{{\"baseline_metric\":\"filed terminated pending caseflow\",\"baseline_value_or_null\":null,\"affected_population_or_exposure_or_null\":null,\"problem_boundary\":\"synthetic aggregate segments\",\"resolution_bps\":{},\"pending_change\":{},\"longest_median_days\":{}}},\"intervention\":{{\"mechanism\":null,\"implementing_owner\":null,\"eligibility_rule\":null,\"exclusions\":\"no individual or judicial decision\",\"existing_treatment_or_programmed_work\":null}},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"effect_population\":\"aggregate matters\",\"horizon\":\"annual illustration\",\"uncertainty\":\"not estimated\",\"transferability_boundary\":\"synthetic only\"}},\"service_floors\":{{\"access\":\"counsel access reported\",\"quality_safety\":\"liberty and victim protection reported\",\"equity_distribution\":\"disparity ratio reported\",\"adequacy_resilience\":\"not yet evaluated\",\"delivery_feasibility\":\"not yet evaluated\",\"counsel_bps\":{},\"victim_notice_bps\":{},\"rights_floor_pass\":{}}},\"costs\":{{\"price_year_or_null\":null,\"gross_cost_or_null\":null,\"implementation_cost_or_null\":null,\"maintenance_cost_or_null\":null,\"offsets_or_null\":null,\"dedicated_receipts_or_null\":null,\"state_local_private_shift_or_null\":null,\"net_cost_or_null\":null,\"public_savings\":null}},\"fiscal_bridge\":{{\"gross_public_funding_need_or_null\":null,\"delivery_efficiency_public_savings_or_null\":null,\"external_economic_benefit_or_null\":null,\"operator_or_private_revenue_or_null\":null,\"legally_dedicated_public_receipts_or_null\":null,\"collection_and_financing_cost_or_null\":null,\"net_public_fiscal_pressure_or_null\":null,\"revenue_authority\":\"none\",\"demand_and_incidence_basis\":\"not established\",\"netting_rule\":\"no values admitted\"}},\"adaptive_pathways\":{{\"pathway_classes\":\"caseflow service only\",\"peer_goal_basis\":null,\"evaluation_horizons\":\"annual\",\"realization_owner_or_null\":null,\"transition_and_implementation_cost_or_null\":null,\"uncertainty_and_downside\":\"unbounded\",\"service_floor_and_distribution_result\":\"held\",\"overlap_and_non_additivity\":\"not reconciled\",\"observation_cadence\":null,\"reopen_triggers\":\"official bounded candidate\",\"current_disposition\":\"held\"}},\"delivery\":{{\"capacity\":null,\"schedule\":null,\"milestones\":null,\"useful_life\":null,\"sunset_or_review\":\"review before use\"}},\"overlap\":{{\"shared_projects\":null,\"shared_cost_allocation\":null,\"other_lane_interactions\":\"ISF VET\",\"non_additivity_rule\":\"no automatic addition\"}},\"readiness\":{{\"domain_evidence_ready\":true,\"candidate_bounded\":false,\"outcome_ready\":false,\"cost_ready\":false,\"floors_ready\":false,\"delivery_ready\":false,\"overlap_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"domain_finding_allowed\":true,\"candidate_recommendation_allowed\":false,\"individual_decision_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}",
        result.resolution_bps, result.pending_change, result.longest_median_days,
        result.counsel_floor_bps, result.victim_notice_floor_bps, result.rights_floor_pass
    )
}

fn run(args: &[String]) -> Result<String, String> {
    let [command, path] = args else {
        return Err(
            "usage: tribunal <analyze|held-pack|official-baseline|official-held-pack> <fixture.tsv>"
                .into(),
        );
    };
    let input = fs::read_to_string(path).map_err(|error| format!("{path}: {error}"))?;
    if !input.contains("# source_id=") || !input.contains("# evidence_label=") {
        return Err("fixture must declare source_id and evidence_label".into());
    }
    if command.starts_with("official-") {
        return official::run(command, &input);
    }
    let result = analyze(&parse(&input)?);
    match command.as_str() {
        "analyze" => Ok(analysis_json(&result)),
        "held-pack" => Ok(held_pack_json(&result)),
        _ => Err(format!("unknown command: {command}")),
    }
}

fn main() -> ExitCode {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str = include_str!("../fixtures/cedar-caseflow.tsv");

    #[test]
    fn reconciles_caseflow_and_reports_resolution() {
        let result = analyze(&parse(FIXTURE).unwrap());
        assert!(result.accounting_reconciles);
        assert_eq!(result.filed, 16_800);
        assert_eq!(result.terminated, 15_150);
        assert_eq!(result.pending_change, 1_650);
    }

    #[test]
    fn preserves_distinct_rights_floors() {
        let result = analyze(&parse(FIXTURE).unwrap());
        assert_eq!(result.counsel_floor_bps, 8_400);
        assert_eq!(result.victim_notice_floor_bps, 7_900);
        assert_eq!(result.longest_detention_days, 41);
        assert_eq!(result.worst_disparity_milli, 1_280);
        assert!(!result.rights_floor_pass);
    }

    #[test]
    fn throughput_does_not_override_rights() {
        let json = analysis_json(&analyze(&parse(FIXTURE).unwrap()));
        assert!(json.contains("\"throughput_alone_is_justice\":false"));
    }

    #[test]
    fn detects_nonreconciling_pending_count() {
        let changed = FIXTURE.replacen("5000\t6200", "5000\t6100", 1);
        assert!(!analyze(&parse(&changed).unwrap()).accounting_reconciles);
    }

    #[test]
    fn held_pack_preserves_judicial_and_taxlane_authority() {
        let pack = held_pack_json(&analyze(&parse(FIXTURE).unwrap()));
        assert!(pack.contains("\"track\":\"JUS\""));
        assert!(pack.contains("\"individual_decision_allowed\":false"));
        assert!(pack.contains("\"public_savings\":null"));
        assert!(pack.contains("\"taxlane_admission_ready\":false"));
    }
}
