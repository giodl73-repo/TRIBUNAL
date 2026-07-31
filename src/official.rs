#[derive(Debug, Clone, PartialEq, Eq)]
struct Flow {
    segment: String,
    pending_start: i64,
    filed: i64,
    terminated: i64,
    pending_end: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct Baseline {
    segments: usize,
    pending_start: i64,
    filed: i64,
    terminated: i64,
    pending_end: i64,
    pending_change: i64,
    reported_adjustment: i64,
    largest_absolute_adjustment_segment: String,
    largest_absolute_adjustment: i64,
}

fn parse(input: &str) -> Result<Vec<Flow>, String> {
    let mut rows = Vec::new();
    for (index, line) in input.lines().enumerate() {
        let line_number = index + 1;
        if line.is_empty() || line.starts_with('#') || line.starts_with("segment\t") {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 5 {
            return Err(format!("line {line_number}: expected 5 fields"));
        }
        let number = |field: usize| {
            fields[field]
                .parse::<i64>()
                .map_err(|_| format!("line {line_number}: invalid integer"))
        };
        let row = Flow {
            segment: fields[0].to_owned(),
            pending_start: number(1)?,
            filed: number(2)?,
            terminated: number(3)?,
            pending_end: number(4)?,
        };
        if row.pending_start < 0 || row.filed < 0 || row.terminated < 0 || row.pending_end < 0 {
            return Err(format!("line {line_number}: counts must be nonnegative"));
        }
        rows.push(row);
    }
    if rows.is_empty() {
        return Err("at least one official caseflow segment is required".into());
    }
    Ok(rows)
}

fn adjustment(row: &Flow) -> i64 {
    row.pending_end - (row.pending_start + row.filed - row.terminated)
}

fn analyze(rows: &[Flow]) -> Baseline {
    let largest = rows
        .iter()
        .max_by_key(|row| adjustment(row).abs())
        .expect("nonempty official rows");
    let pending_start = rows.iter().map(|row| row.pending_start).sum();
    let filed = rows.iter().map(|row| row.filed).sum();
    let terminated = rows.iter().map(|row| row.terminated).sum();
    let pending_end = rows.iter().map(|row| row.pending_end).sum();
    Baseline {
        segments: rows.len(),
        pending_start,
        filed,
        terminated,
        pending_end,
        pending_change: pending_end - pending_start,
        reported_adjustment: pending_end - (pending_start + filed - terminated),
        largest_absolute_adjustment_segment: largest.segment.clone(),
        largest_absolute_adjustment: adjustment(largest).abs(),
    }
}

fn baseline_json(result: &Baseline) -> String {
    format!(
        "{{\"schema\":\"tribunal.official-caseflow-baseline.v1\",\"source_id\":\"AOUSC-FJCS-2025\",\"evidence_label\":\"official_aggregate\",\"segments\":{},\"pending_start\":{},\"filed\":{},\"terminated\":{},\"pending_end\":{},\"pending_change\":{},\"reported_adjustment\":{},\"largest_absolute_adjustment_segment\":\"{}\",\"largest_absolute_adjustment\":{},\"adjustment_is_error\":false,\"workload_proves_rights_quality\":false}}",
        result.segments,
        result.pending_start,
        result.filed,
        result.terminated,
        result.pending_end,
        result.pending_change,
        result.reported_adjustment,
        result.largest_absolute_adjustment_segment,
        result.largest_absolute_adjustment
    )
}

fn held_pack_json(result: &Baseline) -> String {
    format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"tribunal:aousc-fjcs-2025-caseflow:v1\",\"track\":\"JUS\",\"domain_repository\":\"TRIBUNAL\",\"candidate_id\":null,\"candidate_name\":null,\"fiscal_owner\":\"TAXLANE\"}},\"scope\":{{\"geography\":\"United States federal judiciary\",\"population_or_network\":\"appeals district civil district criminal defendants and bankruptcy\",\"ownership\":\"federal judiciary\",\"time_basis\":\"12 months ending 2025-03-31\",\"unit_basis\":\"cases or defendants as source-labelled\",\"included\":\"published filed terminated pending toplines\",\"excluded\":\"individual cases rights quality causes and costs\"}},\"source_custody\":{{\"source_id\":\"AOUSC-FJCS-2025\",\"publisher\":\"Administrative Office of the United States Courts\",\"source_path_or_url\":\"https://www.uscourts.gov/data-news/reports/statistical-reports/federal-judicial-caseload-statistics/federal-judicial-caseload-statistics-2025\",\"vintage\":\"2025-03-31\",\"capture_status\":\"derived_from_published_toplines\",\"checksum_or_null\":null}},\"problem\":{{\"baseline_metric\":\"aggregate caseflow reconciliation\",\"baseline_value_or_null\":{},\"affected_population_or_exposure_or_null\":{},\"problem_boundary\":\"a nonzero adjustment preserves source accounting rather than inventing exact identity\",\"reported_adjustment\":{},\"pending_change\":{}}},\"intervention\":{{\"mechanism\":null,\"implementing_owner\":null,\"eligibility_rule\":null,\"exclusions\":\"no judicial or individual decision\",\"existing_treatment_or_programmed_work\":null}},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"effect_population\":\"aggregate federal workload\",\"horizon\":\"one reporting year\",\"uncertainty\":\"published toplines do not explain adjustments or causal effects\",\"transferability_boundary\":\"not a state or local court estimate\"}},\"service_floors\":{{\"access\":null,\"quality_safety\":null,\"equity_distribution\":null,\"adequacy_resilience\":null,\"delivery_feasibility\":null,\"rights_floor_pass\":null}},\"costs\":{{\"price_year_or_null\":null,\"gross_cost_or_null\":null,\"implementation_cost_or_null\":null,\"maintenance_cost_or_null\":null,\"offsets_or_null\":null,\"dedicated_receipts_or_null\":null,\"state_local_private_shift_or_null\":null,\"net_cost_or_null\":null,\"public_savings\":null}},\"fiscal_bridge\":{{\"gross_public_funding_need_or_null\":null,\"delivery_efficiency_public_savings_or_null\":null,\"external_economic_benefit_or_null\":null,\"operator_or_private_revenue_or_null\":null,\"legally_dedicated_public_receipts_or_null\":null,\"collection_and_financing_cost_or_null\":null,\"net_public_fiscal_pressure_or_null\":null,\"revenue_authority\":\"none\",\"demand_and_incidence_basis\":\"not established\",\"netting_rule\":\"workload is not money\"}},\"adaptive_pathways\":{{\"pathway_classes\":\"baseline observation only\",\"peer_goal_basis\":null,\"evaluation_horizons\":\"annual\",\"realization_owner_or_null\":null,\"transition_and_implementation_cost_or_null\":null,\"uncertainty_and_downside\":\"rights and causes unmeasured\",\"service_floor_and_distribution_result\":\"held\",\"overlap_and_non_additivity\":\"court-system units remain distinct\",\"observation_cadence\":\"annual\",\"reopen_triggers\":\"bounded jurisdiction source spine\",\"current_disposition\":\"held\"}},\"delivery\":{{\"capacity\":null,\"schedule\":null,\"milestones\":null,\"useful_life\":null,\"sunset_or_review\":\"refresh with next AOUSC vintage\"}},\"overlap\":{{\"shared_projects\":null,\"shared_cost_allocation\":null,\"other_lane_interactions\":\"ISF VET\",\"non_additivity_rule\":\"cases and defendants are not interchangeable\"}},\"readiness\":{{\"domain_evidence_ready\":true,\"candidate_bounded\":false,\"outcome_ready\":false,\"cost_ready\":false,\"floors_ready\":false,\"delivery_ready\":false,\"overlap_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"domain_finding_allowed\":true,\"candidate_recommendation_allowed\":false,\"individual_decision_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}",
        result.terminated, result.pending_end, result.reported_adjustment, result.pending_change
    )
}

pub fn run(command: &str, input: &str) -> Result<String, String> {
    if !input.contains("# evidence_label=official_aggregate") {
        return Err("official command requires evidence_label=official_aggregate".into());
    }
    let result = analyze(&parse(input)?);
    match command {
        "official-baseline" => Ok(baseline_json(&result)),
        "official-held-pack" => Ok(held_pack_json(&result)),
        _ => Err(format!("unknown official command: {command}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OFFICIAL: &str = include_str!("../fixtures/official/aousc-fjcs-2025-caseflow.tsv");

    #[test]
    fn preserves_published_adjustments_instead_of_forcing_identity() {
        let result = analyze(&parse(OFFICIAL).unwrap());
        assert_eq!(result.pending_start, 1_430_256);
        assert_eq!(result.filed, 915_138);
        assert_eq!(result.terminated, 1_141_724);
        assert_eq!(result.pending_end, 1_204_224);
        assert_eq!(result.pending_change, -226_032);
        assert_eq!(result.reported_adjustment, 554);
        assert_eq!(result.largest_absolute_adjustment_segment, "district_civil");
        assert_eq!(result.largest_absolute_adjustment, 579);
    }

    #[test]
    fn official_pack_holds_rights_cost_and_taxlane_authority() {
        let pack = held_pack_json(&analyze(&parse(OFFICIAL).unwrap()));
        assert!(pack.contains("\"rights_floor_pass\":null"));
        assert!(pack.contains("\"public_savings\":null"));
        assert!(pack.contains("\"taxlane_admission_ready\":false"));
        assert!(pack.contains("\"candidate_id\":null"));
    }

    #[test]
    fn rejects_negative_counts() {
        let changed = OFFICIAL.replacen("31531", "-1", 1);
        assert!(parse(&changed).is_err());
    }
}
