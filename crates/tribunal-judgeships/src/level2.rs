#[derive(Debug, Clone, PartialEq, Eq)]
struct Snapshot {
    filings: u64,
    terminations: u64,
    pending: u64,
    judgeships: u64,
    vacancy_months_tenths: u64,
    weighted_filings: u64,
    felony_defendants: u64,
    immigration_defendants: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct Baseline {
    prior: Snapshot,
    current: Snapshot,
    filings_change_bps: i64,
    pending_change: i64,
    immigration_share_bps: u64,
}

fn parse(input: &str) -> Result<Baseline, String> {
    for marker in [
        "# evidence_label=official_district_case_type_baseline",
        "# district=Texas Southern",
        "# case_type=criminal_felony_immigration_defendants",
        "# legislative_status=ordered_reported_house_committee_not_enacted",
    ] {
        if !input.contains(marker) {
            return Err(format!("missing required marker: {marker}"));
        }
    }
    let mut rows = Vec::new();
    for (index, line) in input.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') || line.starts_with("period_end\t") {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 9 {
            return Err(format!("line {}: expected 9 fields", index + 1));
        }
        let number = |field: usize| {
            fields[field]
                .parse::<u64>()
                .map_err(|_| format!("line {}: invalid integer", index + 1))
        };
        rows.push((
            fields[0],
            Snapshot {
                filings: number(1)?,
                terminations: number(2)?,
                pending: number(3)?,
                judgeships: number(4)?,
                vacancy_months_tenths: number(5)?,
                weighted_filings: number(6)?,
                felony_defendants: number(7)?,
                immigration_defendants: number(8)?,
            },
        ));
    }
    if rows.len() != 2 || rows[0].0 != "2025-03-31" || rows[1].0 != "2026-03-31" {
        return Err("baseline requires ordered 2025-03-31 and 2026-03-31 rows".into());
    }
    if rows
        .iter()
        .any(|(_, row)| row.immigration_defendants > row.felony_defendants)
    {
        return Err("immigration defendants cannot exceed felony defendants".into());
    }
    let prior = rows[0].1.clone();
    let current = rows[1].1.clone();
    Ok(Baseline {
        filings_change_bps: (current.filings as i64 - prior.filings as i64) * 10_000
            / prior.filings as i64,
        pending_change: current.pending as i64 - prior.pending as i64,
        immigration_share_bps: current.immigration_defendants * 10_000 / current.felony_defendants,
        prior,
        current,
    })
}

fn baseline_json(result: &Baseline) -> String {
    format!(
        "{{\"schema\":\"tribunal.district-case-type-baseline.v1\",\"district\":\"Texas Southern\",\"case_type\":\"criminal_felony_immigration_defendants\",\"period_end\":\"2026-03-31\",\"filings\":{},\"filings_change_bps\":{},\"terminations\":{},\"pending\":{},\"pending_change\":{},\"authorized_judgeships\":{},\"vacant_judgeship_months_tenths\":{},\"weighted_filings_per_judgeship\":{},\"criminal_felony_defendants\":{},\"immigration_felony_defendants\":{},\"immigration_share_bps\":{},\"counsel_access_bps\":null,\"pretrial_detention_days\":null,\"victim_notice_bps\":null,\"disparity_measure\":null,\"candidate_effect\":null,\"rights_floor_pass\":null,\"capacity_is_outcome\":false}}",
        result.current.filings, result.filings_change_bps, result.current.terminations,
        result.current.pending, result.pending_change, result.current.judgeships,
        result.current.vacancy_months_tenths, result.current.weighted_filings,
        result.current.felony_defendants, result.current.immigration_defendants,
        result.immigration_share_bps
    )
}

fn held_pack_json(result: &Baseline) -> String {
    format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"tribunal:txsd-immigration-caseflow-2026:v1\",\"track\":\"JUS\",\"domain_repository\":\"TRIBUNAL\",\"candidate_id\":\"hr1702_judges_act_2025\",\"candidate_name\":\"H.R. 1702 JUDGES Act of 2025\",\"fiscal_owner\":\"TAXLANE\"}},\"scope\":{{\"geography\":\"U.S. District Court for the Southern District of Texas\",\"population_or_network\":\"aggregate district workload and immigration felony defendants\",\"ownership\":\"AOUSC and the federal judiciary; candidate requires Congress President Senate and appropriations\",\"time_basis\":\"12 months ending 2026-03-31 compared with 2025-03-31\",\"unit_basis\":\"filings cases defendant filings judgeships months and basis points\",\"included\":\"district workload and one published criminal case type\",\"excluded\":\"people individual cases merits dispositions and state local tribal courts\"}},\"source_custody\":{{\"source_id\":\"AOUSC-FCMS-TXSD-2026\",\"publisher\":\"Administrative Office of the United States Courts\",\"source_path_or_url\":\"https://www.uscourts.gov/sites/default/files/document/fcms_na_distprofile0331.2026.pdf\",\"vintage\":\"2026-03-31\",\"capture_status\":\"transcribed_official_district_profile_with_reconciliation_tests\",\"checksum_or_null\":null,\"legislative_source_id\":\"CONGRESS-HR1702-119\"}},\"problem\":{{\"baseline_metric\":\"district filings pending workload and immigration felony-defendant mix\",\"baseline_value_or_null\":{},\"affected_population_or_exposure_or_null\":{},\"problem_boundary\":\"workload context only; does not identify delay cause or candidate effect\",\"filings_change_bps\":{},\"pending_change\":{},\"weighted_filings_per_judgeship\":{},\"vacant_judgeship_months_tenths\":{}}},\"intervention\":{{\"mechanism\":\"proposed phased federal district judgeships\",\"implementing_owner\":\"Congress President Senate AOUSC and affected district court\",\"eligibility_rule\":\"district allocations require enacted text and appointments\",\"exclusions\":\"no case assignment adjudication prosecution detention or sentencing recommendation\",\"existing_treatment_or_programmed_work\":\"19 authorized judgeships in the profile\"}},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"effect_population\":\"not established\",\"horizon\":\"not established\",\"uncertainty\":\"observational workload baseline cannot identify a judgeship effect\",\"transferability_boundary\":\"Texas Southern district and published aggregate case type only\"}},\"service_floors\":{{\"access\":\"counsel access unavailable\",\"quality_safety\":\"victim notice and durable resolution unavailable\",\"equity_distribution\":\"disparity measure unavailable\",\"adequacy_resilience\":\"46.0 vacant judgeship months and 801 weighted filings per judgeship are context not a pass threshold\",\"delivery_feasibility\":\"enactment appropriation appointment staffing facilities security and technology unresolved\",\"counsel_access_bps\":null,\"pretrial_detention_days\":null,\"victim_notice_bps\":null,\"disparity_measure\":null,\"rights_floor_pass\":null}},\"costs\":{{\"price_year_or_null\":null,\"gross_cost_or_null\":null,\"implementation_cost_or_null\":null,\"maintenance_cost_or_null\":null,\"offsets_or_null\":null,\"dedicated_receipts_or_null\":null,\"state_local_private_shift_or_null\":null,\"net_cost_or_null\":null,\"public_savings\":null}},\"fiscal_bridge\":{{\"gross_public_funding_need_or_null\":null,\"delivery_efficiency_public_savings_or_null\":null,\"external_economic_benefit_or_null\":null,\"operator_or_private_revenue_or_null\":null,\"legally_dedicated_public_receipts_or_null\":null,\"collection_and_financing_cost_or_null\":null,\"net_public_fiscal_pressure_or_null\":null,\"revenue_authority\":\"none\",\"demand_and_incidence_basis\":\"district workload baseline only\",\"netting_rule\":\"do not infer costs or savings from workload\"}},\"adaptive_pathways\":{{\"pathway_classes\":\"district capacity comparison baseline\",\"peer_goal_basis\":null,\"evaluation_horizons\":\"annual after enactment staffing and stable measurement\",\"realization_owner_or_null\":null,\"transition_and_implementation_cost_or_null\":null,\"uncertainty_and_downside\":\"composition policy appointments and supporting capacity can change workload\",\"service_floor_and_distribution_result\":\"held_missing_rights_measures\",\"overlap_and_non_additivity\":\"immigration defendant filings are contained within felony defendants; do not add\",\"observation_cadence\":\"annual AOUSC profile\",\"reopen_triggers\":\"enactment staffed seats and matched counsel liberty victim disparity and outcome observations\",\"current_disposition\":\"held\"}},\"delivery\":{{\"capacity\":\"19 authorized judgeships in baseline\",\"schedule\":null,\"milestones\":null,\"useful_life\":null,\"sunset_or_review\":\"annual district profile review\"}},\"overlap\":{{\"shared_projects\":\"prosecution defense detention corrections court staff facilities security and technology\",\"shared_cost_allocation\":null,\"other_lane_interactions\":\"JUS DIS ISF\",\"non_additivity_rule\":\"case-type defendants are a subset of criminal defendants and workload is not savings\"}},\"readiness\":{{\"domain_evidence_ready\":true,\"candidate_bounded\":true,\"outcome_ready\":false,\"cost_ready\":false,\"floors_ready\":false,\"delivery_ready\":false,\"overlap_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"domain_finding_allowed\":true,\"candidate_recommendation_allowed\":false,\"individual_decision_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}",
        result.current.pending, result.current.immigration_defendants,
        result.filings_change_bps, result.pending_change, result.current.weighted_filings,
        result.current.vacancy_months_tenths
    )
}

pub fn run(command: &str, input: &str) -> Result<String, String> {
    let result = parse(input)?;
    match command {
        "level2-baseline" => Ok(baseline_json(&result)),
        "level2-held-pack" => Ok(held_pack_json(&result)),
        _ => Err(format!("unknown level 2 command: {command}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str =
        include_str!("../../../fixtures/official/aousc-texas-southern-2026-rights-baseline.tsv");

    #[test]
    fn computes_current_workload_and_case_mix() {
        let result = parse(FIXTURE).unwrap();
        assert_eq!(result.filings_change_bps, 3768);
        assert_eq!(result.pending_change, 3213);
        assert_eq!(result.immigration_share_bps, 8251);
        assert_eq!(result.current.weighted_filings, 801);
    }

    #[test]
    fn preserves_rights_and_causality_nulls() {
        let output = baseline_json(&parse(FIXTURE).unwrap());
        for field in [
            "counsel_access_bps",
            "pretrial_detention_days",
            "victim_notice_bps",
            "disparity_measure",
            "candidate_effect",
            "rights_floor_pass",
        ] {
            assert!(output.contains(&format!("\"{field}\":null")));
        }
    }

    #[test]
    fn held_pack_is_not_admissible_or_savings() {
        let output = held_pack_json(&parse(FIXTURE).unwrap());
        assert!(output.contains("\"current_disposition\":\"held\""));
        assert!(output.contains("\"taxlane_admission_ready\":false"));
        assert!(output.contains("\"public_savings\":null"));
    }

    #[test]
    fn rejects_case_type_larger_than_total() {
        let changed = FIXTURE.replacen("9754\t8049", "9754\t9800", 1);
        assert!(parse(&changed).is_err());
    }

    #[test]
    fn rejects_unenacted_status_omission() {
        let changed = FIXTURE.replace(
            "# legislative_status=ordered_reported_house_committee_not_enacted\n",
            "",
        );
        assert!(parse(&changed).is_err());
    }
}
